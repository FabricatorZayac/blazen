#![no_std]
#![no_main]

#![allow(static_mut_refs)]
#![feature(debug_closure_helpers)]

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    tracef!("Panic: {:?}", info);
    unreachable!()
}

mod button;
mod card;
mod gfx;
mod animator;
mod util;
mod scene;
mod message;
mod linalg;

use core::{mem::MaybeUninit, panic::PanicInfo};
use card::state::CardState;
use message::{Message, MessageBuffer, MessageHandler, Reader};
use scene::{Demo, Menu, Scene, ScenePtr, DEMO};
use util::{Entropy, FrameCounter, MouseCompound};
use wasm4::{self as w4, control::{Mouse, MouseState}, draw::{Color, Framebuffer}, tracef};

struct Blazen {
    fb: Framebuffer,

    prev_mouse: MouseState,
    mouse: Mouse,

    scene: &'static mut dyn Scene,
}

static mut FORMAT_BUF: MaybeUninit<[u8; 200]> = MaybeUninit::uninit();
impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        res.logger.init(unsafe {FORMAT_BUF.assume_init_mut()});

        Menu::init();

        tracef!("Hello {}!", "logger");
        tracef!("__heap_base: {:?}", &raw const __heap_base);
        tracef!("sizeof CardState: {}", size_of::<CardState>());

        tracef!("sizeof Demo: {}", size_of::<Demo>());
        tracef!("DEMO: {:?}", DEMO);

        Blazen {
            fb: res.framebuffer,

            prev_mouse: res.controls.mouse.state(),
            mouse: res.controls.mouse,

            scene: Menu::get(),
        }
    }
    fn update(&mut self) {
        self.fb.replace_palette([
            Color(0x8f9bf6),
            Color(0x161616),
            Color(0xab4646),
            Color(0xf0f0f0),
        ]);
        let mut msg = MessageBuffer::new();
        let (mut tx, rx) = msg.get_channel();

        let m = MouseCompound::new(&self.mouse, self.prev_mouse);

        self.scene.handle_input(&m, &mut tx);

        self.handle_message(&rx);
        self.scene.handle_message(&rx);

        self.scene.update();

        self.scene.render(&self.fb);

        // cleanup
        self.prev_mouse = self.mouse.state();
        Entropy::update(&self.mouse.state());
        FrameCounter::increment();
    }
}

impl MessageHandler for Blazen {
    fn handle_message(&mut self, rx: &Reader) {
        match rx.read() {
            Some(Message::Start) => {
                DEMO.init();
                self.scene = DEMO.get();
            }
            Some(Message::BackToGame) => self.scene = DEMO.get(),
            _ => (),
        }
    }
}

unsafe extern "C" {
    static mut __heap_base: u8;
}

w4::main! { Blazen }
