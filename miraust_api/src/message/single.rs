pub enum SingleMessage {
    PlainText(String),
    At(u64),
}