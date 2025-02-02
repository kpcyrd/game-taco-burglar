pub mod lock;

use embedded_graphics::{pixelcolor::BinaryColor, primitives::PrimitiveStyle};

const BLACK: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::Off);
const WHITE: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);

pub const fn centered(outer: i32, inner: u32) -> i32 {
    outer / 2 - (inner as i32 / 2)
}
