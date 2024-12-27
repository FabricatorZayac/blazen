#[derive(Clone, Copy, Debug)]
pub enum Message {
    Start,
}

pub static mut MESSAGE_BUF: Option<Message> = None;
