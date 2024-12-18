use bitvec::{order::Msb0, view::{AsBits, AsMutBits}};
use heapless::Vec;
use strum::EnumIter;
use wasm4::draw::DrawIndex;

use crate::gfx::texture::{Texture, TextureColors, TEXTURE_HEIGHT, TEXTURE_WIDTH};

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
        // let buf = &ACE_OF_SPADES;
        let colors = TextureColors::OneBpp([DrawIndex::Fourth, DrawIndex::Second]);

        let bufptr = &raw mut TEXTURE_BUFFER;
        let buf = unsafe { bufptr.as_mut().unwrap() };
        *buf = [0; 1200];

        let bitbuf = buf.as_mut_bits::<Msb0>();

        let font = FONT.map(|byte|{ byte ^ 255 });
        let fontbits = font.as_bits::<Msb0>();

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
                // Rank::Eight => Vec::from_slice(&[
                //     (15 + 60 * 10, false), (40 + 60 * 30, false),
                //     (25 + 60 * 20, false),
                //     (15 + 60 * 30, false), (40 + 60 * 30, false),
                //     (25 + 60 * 40, false),
                //     (15 + 60 * 60, true), (40 + 60 * 60, true),
                // ]).unwrap(),
                // TODO:
                _ => unreachable!(),

            };

            let suit_idx = 99 * self.suit as usize + 99 * 13;

            for (draw_idx, flip) in vec {
                let suit_slice = &fontbits[suit_idx..suit_idx+99];
                for i in 0..11 {
                    let suit_bits = &suit_slice[i*9..i*9+9];
                    bitbuf[i*60+draw_idx..i*60+draw_idx+9].copy_from_bitslice(suit_bits);
                }
            }

            // unsafe {
            //     *wasm4::sys::DRAW_COLORS = 0x24;
            //     wasm4::sys::blit(buf.as_ptr(), 0, 0, TEXTURE_WIDTH, TEXTURE_HEIGHT, wasm4::sys::BLIT_1BPP);
            // }
        }

        [
            Texture {
                buf,
                uv: [
                    (0.0, 0.0).into(),
                    (1.0, 0.0).into(),
                    (0.0, 1.0).into(),
                ],
                colors,
            },
            Texture {
                buf,
                uv: [
                    (1.0, 0.0).into(),
                    (1.0, 1.0).into(),
                    (0.0, 1.0).into(),
                ],
                colors,
            },
        ]
    }
}

static mut TEXTURE_BUFFER: [u8; 1200] = [0; 1200];

// spades_ace
// NOTE: This will be replaced by prodcedural textures later

pub const ACE_OF_SPADES: [u8; 600] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0,
    0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc7, 0x1f,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0xf9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xcf, 0x9f, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xfc, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0x1f, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xfc, 0xf9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xcf, 0x9f, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xfc, 0xf9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xcf, 0x9f, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x87, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0, 0x3f, 0xff, 0xf0, 0xff, 0xff, 0xff, 0xfe,
    0x01, 0xff, 0xff, 0x0f, 0xff, 0xff, 0xff, 0xc0, 0x0f, 0xff, 0xf0, 0xff, 0xff, 0xff, 0xfc, 0x00,
    0xff, 0xff, 0x0f, 0xff, 0xff, 0xff, 0xc4, 0x8f, 0xff, 0xc0, 0x3f, 0xff, 0xff, 0xfe, 0xcd, 0xff,
    0xfc, 0x03, 0xff, 0xff, 0xff, 0xf8, 0x7f, 0xff, 0xc0, 0x3f, 0xff, 0xff, 0xff, 0x03, 0xff, 0xfc,
    0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0, 0x00,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0x00, 0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0x00, 0x3f,
    0xff, 0xff, 0xff, 0xff, 0xf0, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x0f, 0xff,
    0xff, 0xff, 0xff, 0xc0, 0x00, 0x00, 0x3f, 0xff, 0xff, 0xff, 0xfc, 0x00, 0x00, 0x03, 0xff, 0xff,
    0xff, 0xfc, 0x00, 0x00, 0x00, 0x03, 0xff, 0xff, 0xff, 0xc0, 0x00, 0x00, 0x00, 0x3f, 0xff, 0xff,
    0xf0, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xff, 0xff, 0xc0,
    0x00, 0x00, 0x00, 0x00, 0x3f, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x03, 0xff, 0xff, 0xc0, 0x00,
    0x00, 0x00, 0x00, 0x3f, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x03, 0xff, 0xff, 0xc0, 0x00, 0x00,
    0x00, 0x00, 0x3f, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x03, 0xff, 0xff, 0xc0, 0x00, 0x00, 0x00,
    0x00, 0x3f, 0xff, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x03, 0xff, 0xff, 0xc0, 0x00, 0x30, 0xc0, 0x00,
    0x3f, 0xff, 0xfc, 0x00, 0x03, 0x0c, 0x00, 0x03, 0xff, 0xff, 0xf0, 0x03, 0xf0, 0xfc, 0x00, 0xff,
    0xff, 0xff, 0x00, 0x3f, 0x0f, 0xc0, 0x0f, 0xff, 0xff, 0xfc, 0x3f, 0xf0, 0xff, 0xc3, 0xff, 0xff,
    0xff, 0xc3, 0xff, 0x0f, 0xfc, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xf0, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xfc, 0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0,
    0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0x00, 0x03, 0xff, 0xc0, 0xff, 0xff, 0xff, 0xc0, 0x00,
    0x3f, 0xfe, 0x1f, 0xff, 0xff, 0xff, 0x00, 0x0f, 0xff, 0xb3, 0x7f, 0xff, 0xff, 0xf0, 0x00, 0xff,
    0xf1, 0x23, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0,
    0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0x0f,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe1, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3f, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf9, 0xf3, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0x9f, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf9, 0xf3, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0x9f, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf8, 0x03, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0x80, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf9, 0xf3, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x9f, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf8, 0xe3, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xc0, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

// font
const FONT_WIDTH: u32 = 9;
const FONT_HEIGHT: u32 = 11;
// const FONT_FLAGS: u32 = 0; // BLIT_1BPP

const FONT: [u8; 211] = [ 0xc1,0xc0,0x4e,0x0f,0x0f,0x0f,0x0f,0x0f,0x0f,0x0f,0x8f,0xc0,0x10,0x31,0x89,0xe3,0xf1,0xc1,0xf1,0xfe,0x4f,0x03,0x08,0x0e,0x0f,0xe3,0xe1,0xe0,0xe2,0x63,0x23,0x93,0xc8,0x00,0x01,0xf9,0xfc,0x80,0x00,0x07,0xf3,0xf8,0x1e,0x03,0xe0,0xfe,0x3f,0x00,0x30,0x3f,0x8f,0x8f,0x8f,0x8f,0x8f,0x80,0x47,0x87,0xe3,0xf0,0xf2,0x02,0x00,0x00,0x7f,0x3f,0x1f,0x1f,0x1f,0x1f,0x9f,0xcf,0xe7,0xf3,0xe0,0x23,0x83,0xe0,0xe2,0x03,0x83,0x9c,0x8e,0x0f,0x83,0x88,0x0c,0x04,0x78,0x7e,0x3f,0x0f,0xa3,0x98,0x0f,0x8f,0x0e,0x1e,0x1f,0x30,0x90,0x09,0x84,0xc2,0x61,0x30,0x98,0x4c,0x26,0x10,0x0c,0x3e,0x0f,0xcf,0xe7,0xf3,0xf9,0xfc,0xfe,0x4f,0x23,0x10,0x0c,0x0f,0x07,0x39,0x3e,0x3f,0x9f,0xcf,0xe7,0x73,0xb8,0xc9,0x31,0xc0,0x0e,0x0f,0x27,0x33,0x39,0x3c,0x3e,0x07,0x39,0x9e,0xcf,0x23,0x8c,0x1c,0x04,0x70,0x7c,0x3e,0x00,0x00,0x07,0xc3,0xe1,0xf0,0xf9,0xef,0xf7,0xf1,0xf8,0xf8,0x38,0x08,0x00,0x01,0x29,0xf7,0xf1,0xff,0xf3,0x90,0x80,0x00,0x00,0x00,0x80,0xe0,0xf8,0xfe,0xff,0xff,0x1f,0x07,0x83,0xc1,0xd1,0x40,0x00,0x00,0x04,0xa7,0xde,0x00,0xf7,0xf1,0xf8,0xf8,0x38,0x08,0x02,0x03,0x83,0xe3,0xf1,0xfd,0xe0 ];
