use crate::bot::Bot;
use crate::contact::friend::Friend;
use crate::contact::group::Group;
use crate::contact::stranger::Stranger;

#[no_mangle]
extern "Rust" fn bot_find_instance(id: u64) -> Option<Bot> {
    Bot::find_instance(id)
}

#[no_mangle]
extern "Rust" fn bot_get_friend(bot: &Bot, id: u64) -> Option<Friend> {
    bot.get_friend(id)
}

#[no_mangle]
extern "Rust" fn bot_get_group(bot: &Bot, id: u64) -> Option<Group> {
    bot.get_group(id)
}

#[no_mangle]
extern "Rust" fn bot_get_stranger(bot: &Bot, id: u64) -> Option<Stranger> {
    bot.get_stranger(id)
}