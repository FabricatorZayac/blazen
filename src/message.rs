use core::cell::Cell;

use wasm4::tracef;

use crate::util::MouseCompound;

pub trait InputHandler {
    fn handle_input(&self, mouse: &MouseCompound, tx: &mut Writer);
}

pub trait MessageHandler {
    fn handle_message(&mut self, rx: &Reader);
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    CardHovered(usize),
    CardClicked(usize),
    // Menu
    Start,
    // Demo
    DeckClicked,
    PlayHand,
    DiscardHand,
    // DeckScene
    BackToGame,
}

// static mut MESSAGE_BUF: Option<Message> = None;

pub struct Writer<'a> {
    message: &'a Cell<Option<Message>>,
    lock: bool,
}
impl<'a> Writer<'a> {
    fn new(message: &'a Cell<Option<Message>>) -> Self {
        Writer {
            message,
            lock: false,
        }
    }
    pub fn write(&mut self, msg: Message) -> Result<(), Message> {
        if self.lock {
            Err(msg)
        } else {
            self.lock = true;
            self.message.replace(Some(msg));
            Ok(())
        }
    }
}

pub struct Reader<'a> {
    message: &'a Cell<Option<Message>>
}
impl<'a> Reader<'a> {
    fn new(message: &'a Cell<Option<Message>>) -> Self {
        Reader { message }
    }
    pub fn read(&self) -> Option<Message> {
        self.message.get()
    }
}
impl<'a> Drop for Reader<'a> {
    fn drop(&mut self) {
        if let Some(msg) = self.read() {
            tracef!("{:?}", msg)
        }
    }
}

pub struct MessageBuffer(Cell<Option<Message>>);
impl MessageBuffer {
    pub fn new() -> Self {
        Self(Cell::new(None))
    }
    pub fn get_channel(&mut self) -> (Writer, Reader) {
        (
            Writer::new(&self.0),
            Reader::new(&self.0),
        )
    }
}
