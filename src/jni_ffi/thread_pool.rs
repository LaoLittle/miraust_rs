use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use jni::JNIEnv;

use crate::jni_ffi::jni_callback::MIRAI_ENV;

pub(crate) struct Worker {
    _thread: JoinHandle<()>,
    status: Arc<Mutex<Status>>,
}

pub(crate) struct Pool {
    threads: Arc<Mutex<Vec<Worker>>>,
    sender: crossbeam::channel::Sender<Job>,
    receiver: crossbeam::channel::Receiver<Job>,
    max_thread: u8,
}

impl Pool {
    pub(crate) fn new(size: u8) -> Pool {
        let (send, recv) = crossbeam::channel::unbounded();
        Pool {
            threads: Arc::new(Mutex::new(Vec::new())),
            sender: send,
            receiver: recv,
            max_thread: size,
        }
    }

    pub(crate) fn execute<F>(&self, fun: F) -> Result<(), Box<dyn Error>>
        where F: FnOnce(&JNIEnv) + Send + 'static
    {
        let arc = self.threads.clone();

        let threads = arc.lock().unwrap();

        for worker in threads.iter() {
            match worker.status() {
                Status::Waiting => {
                    self.sender.send(Box::new(fun))?;
                    return Ok(());
                }
                _ => {}
            }
        }

        if threads.len() >= self.max_thread as usize {
            self.sender.send(Box::new(fun))?;
            return Ok(());
        }

        self.new_worker(Some(Box::new(fun)));
        Ok(())
    }

    fn new_worker(&self, job: Option<Job>) {
        let builder = thread::Builder::new();

        let arc = self.threads.clone();
        let mut threads = arc.lock().unwrap();

        let r = self.receiver.clone();

        let status = Arc::new(Mutex::new(Status::Waiting));
        let _s = status.clone();

        let handle: JoinHandle<()> = builder.name(format!("JNI Pool: Thread {}", threads.len())).spawn(move || {
            let mirai = MIRAI_ENV.get().unwrap();
            let env = mirai.jvm.attach_current_thread_as_daemon().unwrap();

            let status = _s;

            if let Some(job_first) = job {
                job_first(&env);
            }

            while let Ok(job) = r.recv() {
                let mut s = status.lock().unwrap();
                *s = Status::Running;
                drop(s);
                job(&env);
                let mut s = status.lock().unwrap();
                *s = Status::Waiting;
            }

            let mut status = status.lock().unwrap();
            *status = Status::Stopped;
        }).expect("Unable to create thread");

        threads.push(Worker {
            _thread: handle,
            status,
        });
    }
}

impl Worker {
    fn status(&self) -> Status {
        self.status.clone().lock().unwrap().clone()
    }
}

type Job = Box<dyn FnOnce(&JNIEnv) + Send + 'static>;

#[derive(Clone)]
pub(crate) enum Status {
    Waiting,
    Running,
    Stopped,
}