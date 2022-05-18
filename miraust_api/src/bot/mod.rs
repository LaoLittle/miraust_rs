use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;
use crate::jni_struct::GlobalRef;

pub struct Bot {
    pub(crate) id: i64,
    pub(crate) inner: GlobalRef,
}

impl Bot {
    pub const fn id(&self) -> u64 {
        self.id as u64
    }

    pub fn find_instance(id: u64) -> Option<Bot> {
        unsafe { bot_find_instance(id) }
    }

    pub fn get_friend(&self, id: u64) -> Option<Friend> {
        unsafe { bot_get_friend(self, id) }
    }

    pub fn get_group(&self, id: u64) -> Option<Group> {
        unsafe { bot_get_group(self, id) }
    }

    pub fn get_stranger(&self, id: u64) -> Option<Stranger> {
        unsafe { bot_get_stranger(self, id) }
    }
}

#[link(name = "miraust_core")]
extern {
    fn bot_find_instance(id: u64) -> Option<Bot>;

    fn bot_get_friend(bot: &Bot, id: u64) -> Option<Friend>;

    fn bot_get_group(bot: &Bot, id: u64) -> Option<Group>;

    fn bot_get_stranger(bot: &Bot, id: u64) -> Option<Stranger>;
}