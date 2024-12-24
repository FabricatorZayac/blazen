use bit_reverse::ParallelReverse;
use bitvec::{order::Msb0, slice::BitSlice, view::{AsBits, AsMutBits}};
use heapless::Vec;
use strum::EnumIter;
use wasm4::draw::DrawIndex;

use crate::gfx::texture::{Texture, TextureBuffer, TextureColors, TEXTURE_HEIGHT, TEXTURE_WIDTH};

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn value(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Enhancement {
    Point,
    Mult,
    Stone,
    Glass,
}

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    enhancement: Option<Enhancement>,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
            enhancement: None,
        }
    }
    pub fn enhance(&mut self, enhancement: Enhancement) {
        self.enhancement = Some(enhancement);
    }
    pub fn is_face(&self) -> bool {
        match self.rank {
            Rank::Jack | Rank::Queen | Rank::King => true,
            _ => false,
        }
    }
    pub fn texture(&self) -> [Texture; 2] {
        // tracef!("Building texture {:?} of {:?}s", self.rank, self.suit);
        // let buf = &ACE_OF_SPADES;

        TextureBuffer::init();
        let mut buf = TextureBuffer::get_mut();
        let bitbuf = buf.as_mut_bits::<Msb0>();

        font_into_buffer(bitbuf, self.rank as usize, 60 + 2, false);
        font_into_buffer(
            bitbuf,
            self.rank as usize,
            4800 - TEXTURE_WIDTH * (FONT_HEIGHT + 1) - FONT_WIDTH - 2,
            true,
        );

        if !self.is_face() && self.rank != Rank::Ace {
            let vec: Vec<(usize, bool), 10> = match self.rank {
                Rank::Two => {
                    Vec::from_slice(&[
                        (25 + 60 * 10, false),
                        (25 + 60 * 60, true),
                    ]).unwrap()
                },
                // Rank::Three => Vec::from_slice(&[
                //     (25 + 60 * 10, false),
                //     (25 + 60 * 30, false),
                //     (25 + 60 * 60, true),
                // ]).unwrap(),
                // Rank::Four => Vec::from_slice(&[
                //     (15 + 60 * 10, false), (40 + 60 * 30, false),
                //     (15 + 60 * 60, true), (40 + 60 * 60, true),
                // ]).unwrap(),
                // Rank::Five => Vec::from_slice(&[
                //     (15 + 60 * 10, false), (40 + 60 * 30, false),
                //     (25 + 60 * 30, false),
                //     (15 + 60 * 60, true), (40 + 60 * 60, true),
                // ]).unwrap(),
                // Rank::Six => Vec::from_slice(&[
                //     (15 + 60 * 10, false), (40 + 60 * 30, false),
                //     (15 + 60 * 30, false), (40 + 60 * 30, false),
                //     (15 + 60 * 60, true), (40 + 60 * 60, true),
                // ]).unwrap(),
                // Rank::Seven => Vec::from_slice(&[
                //     (15 + 60 * 10, false), (40 + 60 * 30, false),
                //     (25 + 60 * 20, false),
                //     (15 + 60 * 30, false), (40 + 60 * 30, false),
                //     (15 + 60 * 60, true), (40 + 60 * 60, true),
                // ]).unwrap(),
                Rank::Eight =>  {
                    let x1 = 15;
                    let x2 = 60 - 15 - FONT_WIDTH;
                    let x3 = TEXTURE_WIDTH as usize / 2 - (FONT_WIDTH / 2);
                    Vec::from_slice(&[
                        (x1 + 60 * 9, false), (x2 + 60 * 9, false),
                        (x3 + 60 * 20, false),
                        (x1 + 60 * 35, false), (x2 + 60 * 35, false),
                        (x3 + 60 * 49, true),
                        (x1 + 60 * 61, true), (x2 + 60 * 61, true),
                    ]).unwrap()
                },
                // TODO:
                _ => unreachable!(),

            };

            // let suit_idx = 99 * self.suit as usize + 99 * 13;

            // trace("drawing pattern...");
            for (draw_idx, flip) in vec {
                font_into_buffer(bitbuf, (self.suit as usize) + 13, draw_idx, flip);
            }
        }

        // unsafe {
        //     blit(&raw const TEXTURE_BUFFER[0], 0, 0, TEXTURE_WIDTH as u32, TEXTURE_HEIGHT as u32, BLIT_1BPP);
        // }

        let colors = match self.suit {
            Suit::Spade | Suit::Club => TextureColors::OneBpp([DrawIndex::Fourth, DrawIndex::Second]),
            Suit::Heart | Suit::Diamond => TextureColors::OneBpp([DrawIndex::Fourth, DrawIndex::Third]),
        };

        [
            Texture {
                uv: [
                    [0.0, 0.0],
                    [1.0, 0.0],
                    [0.0, 1.0],
                ],
                colors,
            },
            Texture {
                uv: [
                    [1.0, 0.0],
                    [1.0, 1.0],
                    [0.0, 1.0],
                ],
                colors,
            },
        ]
    }
}

fn font_into_buffer(bitbuf: &mut BitSlice<u8, Msb0>, font_idx: usize, draw_idx: usize, flip: bool) {
    if draw_idx > (TEXTURE_WIDTH * TEXTURE_HEIGHT) - (FONT_CHARSIZE * 8) {
        return;
    }

    let font = FONT.map(|byte|{ byte ^ 255 });

    let mut charbuf = [0u8; FONT_CHARSIZE];
    charbuf.copy_from_slice(&font[font_idx*FONT_CHARSIZE..(font_idx + 1)*FONT_CHARSIZE]);

    if flip {
        charbuf = charbuf.map(|byte| byte.swap_bits());
        charbuf.reverse();
    }

    let char_bits = charbuf.as_bits();
    for i in 0..FONT_HEIGHT {
        bitbuf[i*TEXTURE_WIDTH+draw_idx..i*TEXTURE_WIDTH+draw_idx+FONT_WIDTH].copy_from_bitslice(&char_bits[i*FONT_WIDTH..(i+1)*FONT_WIDTH]);
    }
}

// font
const FONT_WIDTH: usize = 10;
const FONT_HEIGHT: usize = 12;
const FONT_CHARSIZE: usize = FONT_HEIGHT * FONT_WIDTH / 8;
// const FONT_FLAGS: u32 = 0; // BLIT_1BPP
const FONT: [u8; 255] = [ 0xc0,0xe0,0x11,0xc0,0xf0,0x78,0x7c,0x3e,0x1f,0x0f,0x87,0xc3,0xf0,0x00,0x00,0xc1,0xe0,0x31,0x84,0xf0,0xfc,0x38,0x1e,0x0f,0xf1,0x3e,0x07,0x08,0x07,0x03,0xf8,0xfc,0x3e,0x0f,0x13,0x8c,0xc7,0x33,0xcc,0x00,0x00,0x3f,0x3f,0xcf,0xf3,0x00,0x00,0x03,0xfc,0xff,0x0f,0xc0,0x7c,0x07,0xe0,0xfe,0x0f,0x80,0x06,0x03,0xfc,0x3e,0x1f,0x0f,0x87,0xc3,0xe0,0x10,0xe0,0x7c,0x1f,0x03,0x88,0x07,0x03,0x00,0x00,0x0f,0xf3,0xfc,0xfe,0x3f,0x1f,0x8f,0xc7,0xf3,0xfc,0xff,0x3f,0xcf,0xc0,0xe0,0x11,0xe0,0x78,0x9e,0x70,0x38,0x04,0x78,0x1e,0x07,0x88,0x07,0x03,0xe0,0xe0,0x11,0xe0,0xfc,0x3f,0x07,0xc8,0xe3,0x01,0xf0,0xf8,0x78,0x7c,0x3f,0x38,0x4c,0x03,0x30,0xcc,0x33,0x0c,0xc3,0x30,0xcc,0x33,0x0c,0xc3,0x00,0xe1,0xf0,0x3c,0x0f,0xe7,0xf9,0xfe,0x7f,0x9f,0xe7,0xf9,0x3c,0x46,0x10,0x0e,0x07,0xc0,0xe0,0x11,0xe0,0xfc,0x3f,0x0f,0xc3,0xb0,0xc8,0x30,0x46,0x38,0xc7,0x00,0x1c,0x0f,0x13,0x8c,0xc7,0x23,0xc1,0xf0,0xfc,0x0f,0x21,0xce,0x33,0xc4,0x70,0xc0,0xe0,0x11,0xe0,0xfc,0x3f,0x0f,0xc0,0x00,0x00,0x3f,0x0f,0xc3,0xf0,0xfc,0xf3,0xfc,0xfe,0x1f,0x03,0x80,0x40,0x00,0x00,0x00,0x33,0x3c,0xfe,0x1f,0x03,0xff,0xff,0xf8,0xc4,0x30,0x00,0x00,0x00,0x02,0x01,0xc0,0xf8,0x7f,0x3f,0xff,0xf3,0xf8,0x7e,0x1f,0xcf,0x92,0x40,0x00,0x02,0x49,0xf3,0xfc,0xf8,0x06,0x01,0xf3,0xfc,0xfe,0x1f,0x03,0x80,0x40,0x00,0x02,0x01,0xc0,0xf8,0x7f,0x3f,0xcf ];
