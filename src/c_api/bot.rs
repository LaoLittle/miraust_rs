use crate::bot::bot::Bot;

#[no_mangle]
fn bot_get_instance(id: i64) -> Option<Bot> {
    Bot::get_instance(id)
}