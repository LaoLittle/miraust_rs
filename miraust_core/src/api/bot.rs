use std::ptr::null;
use std::sync::mpsc;
use jni::objects::GlobalRef;

use crate::bot::Bot;
use crate::jni_ffi::jni_callback::call_back;

#[no_mangle]
extern fn bot_find_instance(id: u64) -> *const GlobalRef {
    bot_find_instance0(id).map_or(null(), |g| {
        Box::into_raw(Box::new(g))
    })
}

fn bot_find_instance0(id: u64) -> Option<GlobalRef> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None; };

    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::find_instance_unchecked(env, id) }).unwrap();
    });

    recv.recv().unwrap_or(None)
}

#[no_mangle]
extern fn bot_get_friend(bot: &GlobalRef, id: u64) -> *const GlobalRef {
    bot_get_friend0(bot, id).map_or(null(), |g| {
        Box::into_raw(Box::new(g))
    })
}

fn bot_get_friend0(bot: &GlobalRef, id: u64) -> Option<GlobalRef> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None; };

    let global_ref = bot.clone();
    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::get_friend_unchecked(global_ref, env, id) }).unwrap();
    });

    recv.recv().unwrap()
}

#[no_mangle]
fn bot_get_group(bot: &GlobalRef, id: u64) -> *const GlobalRef {
    bot_get_group0(bot, id).map_or(null(), |g| {
        Box::into_raw(Box::new(g))
    })
}

fn bot_get_group0(bot: &GlobalRef, id: u64) -> Option<GlobalRef> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None; };

    let global_ref = bot.clone();
    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::get_group_unchecked(global_ref, env, id) }).unwrap();
    });

    recv.recv().unwrap()
}

#[no_mangle]
fn bot_get_stranger(bot: &GlobalRef, id: u64) -> *const GlobalRef {
    bot_get_stranger0(bot, id).map_or(null(), |g| {
        Box::into_raw(Box::new(g))
    })
}

fn bot_get_stranger0(bot: &GlobalRef, id: u64) -> Option<GlobalRef> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None; };

    let global_ref = bot.clone();
    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::get_stranger_unchecked(global_ref, env, id) }).unwrap();
    });

    recv.recv().unwrap()
}