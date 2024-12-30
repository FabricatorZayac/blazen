#[derive(Clone, Copy, Debug)]
pub enum Message {
    // Menu
    Start,
    // Demo
    CardClicked(usize),
    DeckClicked,
    // DeckScene
    BackToGame,
}

pub static mut MESSAGE_BUF: Option<Message> = None;
