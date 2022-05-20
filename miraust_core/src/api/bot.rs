use std::ptr::{null};
use std::sync::mpsc;
use crate::bot::Bot;
use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;
use crate::jni_ffi::jni_callback::call_back;

#[no_mangle]
extern fn bot_find_instance(id: u64) -> *const Bot {
    bot_find_instance0(id).map_or(null(), |b| {
        Box::into_raw(Box::new(b))
    })
}

fn bot_find_instance0(id: u64) -> Option<Bot> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::find_instance_unchecked(env, id) }).unwrap();
    });

    let r = recv.recv().unwrap_or(None);

    r.map(|global_ref| Bot { inner: global_ref })
}

#[no_mangle]
extern fn bot_get_friend(bot: &Bot, id: u64) -> *const Friend {
    bot_get_friend0(bot, id).map_or(null(), |f| {
        Box::into_raw(Box::new(f))
    })
}

fn bot_get_friend0(bot: &Bot, id: u64) -> Option<Friend> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

    let global_ref = bot.inner.clone();
    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::get_friend_unchecked(global_ref, env, id) }).unwrap();
    });

    let r = recv.recv().ok()?;

    r.map(|global_ref| Friend { inner: global_ref })
}

#[no_mangle]
fn bot_get_group(bot: &Bot, id: u64) -> *const Group {
    bot_get_group0(bot, id).map_or(null(), |g| {
        Box::into_raw(Box::new(g))
    })
}

fn bot_get_group0(bot: &Bot, id: u64) -> Option<Group> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

    let global_ref = bot.inner.clone();
    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::get_group_unchecked(global_ref, env, id) }).unwrap();
    });

    let r = recv.recv().ok()?;

    r.map(|global_ref| Group { inner: global_ref })
}

#[no_mangle]
fn bot_get_stranger(bot: &Bot, id: u64) -> *const Stranger {
    bot_get_stranger0(bot, id).map_or(null(), |s| {
        Box::into_raw(Box::new(s))
    })
}

fn bot_get_stranger0(bot: &Bot, id: u64) -> Option<Stranger> {
    let id = if id <= i64::MAX as u64 { id as i64 } else { return None };

    let global_ref = bot.inner.clone();
    let (send, recv) = mpsc::channel();
    call_back(move |env| {
        send.send(unsafe { Bot::get_stranger_unchecked(global_ref, env, id) }).unwrap();
    });

    let r = recv.recv().ok()?;

    r.map(|global_ref| Stranger { inner: global_ref })
}