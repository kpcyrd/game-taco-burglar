pub mod lock;

use embedded_graphics::{
    mono_font::{ascii, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    primitives::PrimitiveStyle,
};

pub const DISPLAY_WIDTH: i32 = 128;
pub const DISPLAY_HEIGHT: i32 = 64;

const BLACK: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::Off);
const WHITE: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);

pub const CHAR_WIDTH: usize = 4;
pub const TEXT_STYLE: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_4X6)
    .text_color(BinaryColor::On)
    .build();

pub const fn black_stroke(width: u32) -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(BinaryColor::Off, width)
}

pub const fn white_stroke(width: u32) -> PrimitiveStyle<BinaryColor> {
    PrimitiveStyle::with_stroke(BinaryColor::On, width)
}

pub const fn centered(outer: i32, inner: u32) -> i32 {
    outer / 2 - (inner as i32 / 2)
}

// for some reason we need to subtract 1
// putting this in a function for better context
pub const fn line_tweak(num: i32) -> i32 {
    num - 1
}

pub const fn text_align_right(text: &str, total: u8) -> i32 {
    (total as usize - (text.len() * CHAR_WIDTH)) as i32
}
