use tokio::task::JoinHandle;
use crate::bot::Bot;
use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;
use crate::event::Event;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Managed {
    pub(crate) pointer: *mut (),
    t: u8,
}

impl Managed {
    pub(crate) fn new(ptr: *mut (), t: u8) -> Managed {
        Managed { pointer: ptr, t }
    }
}

#[no_mangle]
extern fn drop_res(res: *mut (), t: u8) {
    unsafe {
        match t {
            0 => {
                Box::from_raw(res as *mut Bot);
            }
            1 => {
                Box::from_raw(res as *mut Friend);
            }
            2 => {
                Box::from_raw(res as *mut Group);
            }
            3 => {
                Box::from_raw(res as *mut Stranger);
            }
            11 => {
                Box::from_raw(res as *mut Listener);
            }
            12 => {
                Box::from_raw(res as *mut Event);
            }
            _ => {

            }
        };
    }
}

type Listener = JoinHandle<()>;