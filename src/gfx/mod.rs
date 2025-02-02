pub mod lock;

use embedded_graphics::{pixelcolor::BinaryColor, primitives::PrimitiveStyle};

#[allow(dead_code)]
const BLACK: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::Off);
const WHITE: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);

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
