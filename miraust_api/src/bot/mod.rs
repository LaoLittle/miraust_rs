
use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;

use crate::managed::Managed;

pub struct Bot {
    pub(crate) inner: Managed,
}

impl Bot {
    pub const fn id(&self) -> u64 {
        todo!()
    }

    pub fn find_instance(id: u64) -> Option<Bot> {
        let ptr = unsafe { bot_find_instance(id) };
        if ptr.is_null() { None } else { Some(Bot { inner: Managed::new(ptr, 0) }) }
    }

    pub fn get_friend(&self, id: u64) -> Option<Friend> {
        let ptr = unsafe { bot_get_friend(self.inner.pointer, id) };
        if ptr.is_null() {None} else { Some(Friend { inner: Managed::new(ptr, 1) }) }
    }

    pub fn get_group(&self, id: u64) -> Option<Group> {
        let ptr =  unsafe { bot_get_group(self.inner.pointer, id) };
        if ptr.is_null() {None} else { Some(Group { inner: Managed::new(ptr, 2) }) }
    }

    pub fn get_stranger(&self, id: u64) -> Option<Stranger> {
        let ptr = unsafe { bot_get_stranger(self.inner.pointer, id) };
        if ptr.is_null() {None} else { Some(Stranger { inner: Managed::new(ptr, 3) }) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn bot_find_instance(id: u64) -> *mut ();

    fn bot_get_friend(bot: *const (), id: u64) -> *mut ();

    fn bot_get_group(bot: *const (), id: u64) -> *mut ();

    fn bot_get_stranger(bot: *const (), id: u64) -> *mut ();
}