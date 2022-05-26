use crate::contact::Contact;
use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;
use crate::managed::Managed;

pub struct Bot(Managed);

impl Bot {
    pub const fn id(&self) -> u64 {
        todo!()
    }

    pub fn find_instance(id: u64) -> Option<Bot> {
        let ptr = unsafe { bot_find_instance(id) };
        if ptr.is_null() { None } else { Some(Bot(Managed::new(ptr, 0))) }
    }

    pub fn get_friend(&self, id: u64) -> Option<Friend> {
        let ptr = unsafe { bot_get_friend(self.0.pointer, id) };
        if ptr.is_null() { None } else { Some(Friend(Contact(Managed::new(ptr, 0)))) }
    }

    pub fn get_group(&self, id: u64) -> Option<Group> {
        let ptr = unsafe { bot_get_group(self.0.pointer, id) };
        if ptr.is_null() { None } else { Some(Group(Contact(Managed::new(ptr, 0)))) }
    }

    pub fn get_stranger(&self, id: u64) -> Option<Stranger> {
        let ptr = unsafe { bot_get_stranger(self.0.pointer, id) };
        if ptr.is_null() { None } else { Some(Stranger(Contact(Managed::new(ptr, 0)))) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn bot_find_instance(id: u64) -> *mut ();

    fn bot_get_friend(bot: *const (), id: u64) -> *mut ();

    fn bot_get_group(bot: *const (), id: u64) -> *mut ();

    fn bot_get_stranger(bot: *const (), id: u64) -> *mut ();
}