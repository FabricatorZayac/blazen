#![no_std]
#![no_main]

#![allow(internal_features, static_mut_refs)]
#![feature(core_intrinsics, coerce_unsized)]

use core::{arch::wasm32::unreachable, cell::Cell, mem::MaybeUninit, panic::PanicInfo};
#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    unreachable()
}

// mod alloc;
mod button;
mod card;
mod gfx;
mod animator;
mod util;
mod scene;
mod message;

use message::Message;
use scene::{Demo, Menu, Scene};
use wasm4::{
    self as w4, control::{Mouse, MouseState}, draw::{Color, Framebuffer}, tracef
};

static mut LOG_BUF: MaybeUninit<[u8; 200]> = MaybeUninit::uninit();

static mut FRAME_COUNT: u32 = 0;
struct FrameCounter;
impl FrameCounter {
    fn get() -> u32 {
        unsafe { FRAME_COUNT }
    }
    fn increment() {
        unsafe { FRAME_COUNT += 1 };
    }
}

static mut ENTROPY: [u8; 16] = [0; 16];
struct Entropy;
impl Entropy {
    fn get() -> [u8; 16] {
        unsafe { ENTROPY }
    }
    fn update(m: &MouseState) {
        unsafe {
            ENTROPY[(FrameCounter::get() % 16) as usize] = (m.x * 10 + m.y) as u8;
        }
    }
}

struct MouseSemaphore {
    mouse: Mouse,
    prev: Option<MouseState>,
    lock: Cell<bool>,
}
impl MouseSemaphore {
    pub fn new(mouse: Mouse) -> Self {
        MouseSemaphore { mouse, prev: None, lock: Cell::new(false) }
    }
    pub fn state(&self) -> Option<MouseState> {
        if self.lock.get() {
            None
        } else {
            Some(self.mouse.state())
        }
    }
    pub fn lock(&self) {
        self.lock.replace(true);
    }
    pub fn update(&mut self) {
        self.unlock(); 
        self.prev = self.state();
    }

    fn unlock(&mut self) {
        self.lock.replace(false);
    }
}

struct Blazen {
    fb: Framebuffer,

    mouse: MouseSemaphore,

    scene: &'static mut dyn Scene,
}

impl w4::rt::Runtime for Blazen {
    fn start(res: w4::rt::Resources) -> Self {
        res.logger.init(unsafe {LOG_BUF.assume_init_mut()});

        tracef!("Hello {}!", "logger");
        tracef!("__heap_base: {:?}", &raw const __heap_base);

        Menu::init();
        Demo::init();

        Blazen {
            fb: res.framebuffer,
            mouse: MouseSemaphore::new(res.controls.mouse),

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
        self.scene.update(&self.mouse);
        if let Some(msg) = unsafe { message::MESSAGE_BUF } {
            match msg {
                Message::Start => {
                    self.scene = Demo::get();
                },
            }
        }

        self.scene.render(&self.fb);

        unsafe {
            if let Some(msg) = message::MESSAGE_BUF {
                tracef!("{:?}", msg)
            }
        }

        self.mouse.update();
        unsafe { message::MESSAGE_BUF = None };
        Entropy::update(&self.mouse.state().unwrap());
        FrameCounter::increment();
    }
}

unsafe extern "C" {
    static __heap_base: u8;
}

w4::main! { Blazen }
