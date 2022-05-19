
use crate::contact::group::Group;

use crate::event::message_event::MessageEvent;

trait GroupEvent {
    fn group(&self) -> Group;
}

struct GroupMessageEvent{
    event: MessageEvent
}

impl GroupMessageEvent {

}

impl GroupEvent for GroupMessageEvent {
    fn group(&self) -> Group {
        todo!()
    }
}