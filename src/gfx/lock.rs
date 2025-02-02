use crate::gfx::{self, BLACK, WHITE};
use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
};

const DISPLAY_WIDTH: i32 = 128;
const DISPLAY_HEIGHT: i32 = 64;

const CIRLCE_DIAMETER: u32 = 40;
const KEYHOLE_WIDTH: u32 = 10;
const KEYHOLE_HEIGHT: u32 = 26;

const PICK_LENGTH: i32 = 25;
const PICK_KEYHOLE_OFFSET: i32 = 5;
const PICK_WIDTH: u32 = 4;

const LOCK_Y_OFFSET: i32 = 12;

pub fn draw_front<D: DrawTarget<Color = BinaryColor>>(display: &mut D)
where
    <D as DrawTarget>::Error: Debug,
{
    // circle
    Circle::new(Point::new(44, LOCK_Y_OFFSET), 40)
        .into_styled(WHITE)
        .draw(display)
        .unwrap();

    // keyhole
    Rectangle::new(
        Point::new(
            gfx::centered(DISPLAY_WIDTH, KEYHOLE_WIDTH),
            gfx::centered(CIRLCE_DIAMETER as i32, KEYHOLE_HEIGHT) + LOCK_Y_OFFSET,
        ),
        Size::new(KEYHOLE_WIDTH, KEYHOLE_HEIGHT),
    )
    .into_styled(BLACK)
    .draw(display)
    .unwrap();

    // pick
    Line::new(
        Point::new(
            gfx::centered(DISPLAY_WIDTH, 0),
            gfx::centered(DISPLAY_HEIGHT, 0) + PICK_KEYHOLE_OFFSET,
        ),
        Point::new(
            gfx::centered(DISPLAY_WIDTH, 0) + PICK_LENGTH,
            gfx::centered(DISPLAY_HEIGHT, 0) + PICK_LENGTH + PICK_KEYHOLE_OFFSET,
        ),
    )
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, PICK_WIDTH))
    .draw(display)
    .unwrap();
}
