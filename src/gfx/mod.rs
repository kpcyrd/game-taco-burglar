pub mod lock;
pub mod start;
pub mod travel;

use core::fmt::Debug;
use embedded_graphics::{
    mono_font::{ascii, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyle,
    text::{Baseline, Text},
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

pub const fn text_align_center(text: &str, total: i32, font: &MonoFont) -> i32 {
    centered(total, text.len() as u32 * font.character_size.width)
}

pub fn render_tacos<D: DrawTarget<Color = BinaryColor>>(display: &mut D, score: u32)
where
    <D as DrawTarget>::Error: Debug,
{
    let style = TEXT_STYLE;

    // unit
    let tacos = " tacos";
    Text::with_baseline(
        tacos,
        Point::new(text_align_right(tacos, DISPLAY_WIDTH as u8), 0),
        style,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();
    let unit_width = tacos.len() as u32 * style.font.character_size.width;
    let remaining_width = DISPLAY_WIDTH - unit_width as i32;

    // score
    let mut buf = itoa::Buffer::new();
    let buf = buf.format(score);
    Text::with_baseline(
        buf,
        Point::new(text_align_right(buf, remaining_width as u8), 0),
        style,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();
}
