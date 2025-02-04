use crate::gfx;
use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle, RoundedRectangle},
    text::{Baseline, Text},
};

// small screen consts
const CIRLCE_DIAMETER: u32 = 40;
const KEYHOLE_WIDTH: u32 = 10;
const KEYHOLE_HEIGHT: u32 = 26;
const KEYHOLE_OPEN_TWIST: i32 = 5;

const TENSION_TOOL_LENGTH: i32 = 25;
const TENSION_TOOL_KEYHOLE_OFFSET: i32 = 5;
const TENSION_TOOL_WIDTH: u32 = 4;

const KEYHOLE_Y_OFFSET: i32 = 12;

// big screen consts
const LOCK_LENGTH: u32 = 95;
const LOCK_HEIGHT: u32 = 40;
const LOCK_X_OFFSET: i32 = 25;
const SIDE_LOCK_Y_OFFSET: i32 = 8;
const LOCK_ROUND_CORNERS: u32 = 6;

const SHEAR_LINE_DISTANCE: u32 = 5;

const NUM_PINS: usize = 5;
const PINS_X_OFFSET: u32 = 20;
const PINS_Y_OFFSET: u32 = 4;
const PIN_WIDTH: u32 = 10;
const PIN_HEIGHT: u32 = KEYWAY_Y_OFFSET - PINS_Y_OFFSET;
const PIN_X_SPACING: u32 = 3;

const KEYWAY_Y_OFFSET: u32 = LOCK_HEIGHT - KEYWAY_HEIGHT - 5;
const KEYWAY_HEIGHT: u32 = 10;
const KEYWAY_LENGTH: u32 = PINS_X_OFFSET + (NUM_PINS as u32 * (PIN_WIDTH + PIN_X_SPACING));

const PICK_WIDTH: u32 = 2;
const PICK_Y_OFFSET: u32 = 3;
const PICK_HOOK_HEIGHT: u32 = 3;

// game constants
#[allow(dead_code)]
const MAX_CHALLENGE_SIZE: u32 = PIN_HEIGHT - SHEAR_LINE_DISTANCE - 2;
static_assertions::const_assert!(MAX_CHALLENGE_SIZE == 14);

// big screen absolute positions
const LOCK_TOP_OFFSET: i32 =
    gfx::centered(gfx::DISPLAY_HEIGHT - SIDE_LOCK_Y_OFFSET, LOCK_HEIGHT) + SIDE_LOCK_Y_OFFSET;
const KEYWAY_TOP_OFFSET: i32 = LOCK_TOP_OFFSET + KEYWAY_Y_OFFSET as i32;
const SHEAR_LINE_TOP_OFFSET: i32 = KEYWAY_TOP_OFFSET - SHEAR_LINE_DISTANCE as i32;
const PINS_TOP_OFFSET: i32 = LOCK_TOP_OFFSET + PINS_Y_OFFSET as i32;
const PINS_LEFT_OFFSET: i32 = LOCK_X_OFFSET + PINS_X_OFFSET as i32;

pub struct LockPin {
    pub state: u8,
    pub solution: u8,
}

impl LockPin {
    pub fn new(solution: u8) -> Self {
        Self { state: 0, solution }
    }
}

pub struct LockState {
    pub open: bool,
    pub score: u32,
    pub pins: [LockPin; NUM_PINS],
    pub current_pin: u8,
}

impl LockState {
    pub fn new() -> Self {
        Self {
            open: false,
            score: 1338,
            pins: [
                LockPin::new(5),
                LockPin::new(2),
                LockPin::new(8),
                LockPin::new(11),
                LockPin::new(2),
            ],
            current_pin: (NUM_PINS - 1) as u8,
        }
    }

    pub fn draw_big_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        // render lock
        RoundedRectangle::with_equal_corners(
            Rectangle::new(
                Point::new(LOCK_X_OFFSET, LOCK_TOP_OFFSET),
                Size::new(LOCK_LENGTH, LOCK_HEIGHT),
            ),
            Size::new(LOCK_ROUND_CORNERS, LOCK_ROUND_CORNERS),
        )
        .into_styled(gfx::WHITE)
        .draw(display)
        .unwrap();

        // render keyway
        Rectangle::new(
            Point::new(LOCK_X_OFFSET, KEYWAY_TOP_OFFSET),
            Size::new(KEYWAY_LENGTH, KEYWAY_HEIGHT),
        )
        .into_styled(gfx::BLACK)
        .draw(display)
        .unwrap();

        // render shear line
        Rectangle::new(
            Point::new(LOCK_X_OFFSET, SHEAR_LINE_TOP_OFFSET),
            Size::new(KEYWAY_LENGTH, 1),
        )
        .into_styled(gfx::BLACK)
        .draw(display)
        .unwrap();

        // render pins
        for (num, pin) in self.pins.iter().enumerate() {
            let num = num as i32;

            let point = Point::new(
                PINS_LEFT_OFFSET + (num * (PIN_WIDTH + PIN_X_SPACING) as i32),
                PINS_TOP_OFFSET,
            );
            Rectangle::new(point, Size::new(PIN_WIDTH, PIN_HEIGHT))
                .into_styled(gfx::BLACK)
                .draw(display)
                .unwrap();

            Rectangle::new(
                point + Point::new(1, 1 + pin.state as i32),
                Size::new(PIN_WIDTH - 2, pin.solution as u32),
            )
            .into_styled(gfx::WHITE)
            .draw(display)
            .unwrap();
        }

        // render pick
        let pick_length = LOCK_X_OFFSET as u32
            + PINS_X_OFFSET
            + (self.current_pin as u32 * (PIN_WIDTH + PIN_X_SPACING))
            + (PIN_WIDTH / 2)
            + (PICK_WIDTH / 2);
        Rectangle::new(
            Point::new(
                0,
                KEYWAY_TOP_OFFSET + (PICK_Y_OFFSET + PICK_HOOK_HEIGHT) as i32,
            ),
            Size::new(pick_length, PICK_WIDTH),
        )
        .into_styled(gfx::WHITE)
        .draw(display)
        .unwrap();

        // render pick hook
        Rectangle::new(
            Point::new(
                (pick_length - PICK_WIDTH) as i32,
                KEYWAY_TOP_OFFSET + PICK_Y_OFFSET as i32,
            ),
            Size::new(PICK_WIDTH, PICK_HOOK_HEIGHT),
        )
        .into_styled(gfx::WHITE)
        .draw(display)
        .unwrap();

        // render score
        let mut buf = itoa::Buffer::new();
        let buf = buf.format(self.score);
        Text::with_baseline(
            buf,
            Point::new(gfx::text_align_right(buf, gfx::DISPLAY_WIDTH as u8), 0),
            gfx::TEXT_STYLE,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }

    pub fn draw_small_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        let twist = self.open.then_some(KEYHOLE_OPEN_TWIST).unwrap_or_default();

        // circle
        Circle::new(Point::new(44, KEYHOLE_Y_OFFSET), 40)
            .into_styled(gfx::WHITE)
            .draw(display)
            .unwrap();

        // keyhole
        Line::new(
            Point::new(
                gfx::line_tweak(gfx::centered(gfx::DISPLAY_WIDTH, 0)) + twist,
                gfx::centered(CIRLCE_DIAMETER as i32, KEYHOLE_HEIGHT) + KEYHOLE_Y_OFFSET,
            ),
            Point::new(
                gfx::line_tweak(gfx::centered(gfx::DISPLAY_WIDTH, 0)) - twist,
                gfx::line_tweak(
                    gfx::centered(CIRLCE_DIAMETER as i32, KEYHOLE_HEIGHT)
                        + KEYHOLE_Y_OFFSET
                        + KEYHOLE_HEIGHT as i32,
                ),
            ),
        )
        .into_styled(gfx::black_stroke(KEYHOLE_WIDTH))
        .draw(display)
        .unwrap();

        // pick
        Line::new(
            Point::new(
                gfx::centered(gfx::DISPLAY_WIDTH, 0) - twist,
                gfx::centered(gfx::DISPLAY_HEIGHT, 0) + TENSION_TOOL_KEYHOLE_OFFSET,
            ),
            Point::new(
                gfx::centered(gfx::DISPLAY_WIDTH, 0) + TENSION_TOOL_LENGTH - (twist * 5),
                gfx::centered(gfx::DISPLAY_HEIGHT, 0)
                    + TENSION_TOOL_LENGTH
                    + TENSION_TOOL_KEYHOLE_OFFSET,
            ),
        )
        .into_styled(gfx::white_stroke(TENSION_TOOL_WIDTH))
        .draw(display)
        .unwrap();
    }
}
