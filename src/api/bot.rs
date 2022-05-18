use crate::bot::bot::Bot;
use crate::contact::friend::Friend;

#[no_mangle]
extern "Rust" fn bot_get_instance(id: i64) -> Option<Bot> {
    Bot::get_instance(id)
}

#[no_mangle]
extern "Rust" fn bot_get_friend(bot: &Bot, id: i64) -> Option<Friend> {
    bot.get_friend(id)
}