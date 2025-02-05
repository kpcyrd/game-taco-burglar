use crate::game::Screen;
use crate::gfx;
use core::cmp;
use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle, RoundedRectangle},
};
use rand::Rng;
use rand_core::RngCore;

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
const MIN_CHALLENGE_SIZE: u32 = 5;
const MAX_CHALLENGE_SIZE: u32 = PIN_HEIGHT - SHEAR_LINE_DISTANCE - 4;
static_assertions::const_assert!(MAX_CHALLENGE_SIZE == 12);
const PICK_SPEED: u8 = 1;
const SOLVE_TOLERANCE: u32 = 2;

const MIN_SCORE_REWARD: u32 = 100;
const MAX_SCORE_REWARD: u32 = 250;

const SOLVE_COOLDOWN: u8 = 5;

// big screen absolute positions
const LOCK_TOP_OFFSET: i32 =
    gfx::centered(gfx::DISPLAY_HEIGHT - SIDE_LOCK_Y_OFFSET, LOCK_HEIGHT) + SIDE_LOCK_Y_OFFSET;
const KEYWAY_TOP_OFFSET: i32 = LOCK_TOP_OFFSET + KEYWAY_Y_OFFSET as i32;
const SHEAR_LINE_TOP_OFFSET: i32 = KEYWAY_TOP_OFFSET - SHEAR_LINE_DISTANCE as i32;
const PINS_TOP_OFFSET: i32 = LOCK_TOP_OFFSET + PINS_Y_OFFSET as i32;
const PINS_LEFT_OFFSET: i32 = LOCK_X_OFFSET + PINS_X_OFFSET as i32;

pub enum Direction {
    Up,
    Down,
}

pub struct LockPin {
    pub state: u8,
    pub height: u8,
    pub direction: Direction,
}

impl LockPin {
    pub fn random<R: RngCore>(mut random: R) -> Self {
        let height = random.gen_range(MIN_CHALLENGE_SIZE..=MAX_CHALLENGE_SIZE) as u8;
        Self {
            state: 0,
            height,
            direction: Direction::Down,
        }
    }

    pub const fn is_near_shear(&self) -> bool {
        let total_pin = (self.state + self.height) as u32;

        // check the distance to the shear line
        // always return false if we're inside the core
        let Some(distance) = (PIN_HEIGHT - SHEAR_LINE_DISTANCE).checked_sub(total_pin) else {
            return false;
        };

        // still reaches inside the core
        if distance == 0 {
            return false;
        }

        // check if within acceptable range
        distance <= SOLVE_TOLERANCE
    }
}

pub struct LockState {
    pub open: bool,
    pub score: u32,
    pub reward: u32,
    pub pins: [LockPin; NUM_PINS],
    pub current_pin: u8,
    pub solve_cooldown: u8,
    pub transition: Option<Screen>,
}

impl LockState {
    pub fn new<R: RngCore>(score: u32, mut random: R) -> Self {
        Self {
            open: false,
            score,
            reward: random.gen_range(MIN_SCORE_REWARD..=MAX_SCORE_REWARD),
            pins: [
                LockPin::random(&mut random),
                LockPin::random(&mut random),
                LockPin::random(&mut random),
                LockPin::random(&mut random),
                LockPin::random(&mut random),
            ],
            current_pin: (NUM_PINS - 1) as u8,
            solve_cooldown: SOLVE_COOLDOWN,
            transition: None,
        }
    }

    fn current_pin(&mut self) -> &mut LockPin {
        self.current_pin %= NUM_PINS as u8;
        &mut self.pins[self.current_pin as usize]
    }

    pub fn tick(&mut self) {
        if self.open {
            self.solve_cooldown = self.solve_cooldown.saturating_sub(1);
            if self.solve_cooldown == 0 {
                self.transition = Some(Screen::Travel);
            }
        } else {
            let pin = self.current_pin();
            pin.state = match pin.direction {
                Direction::Up => pin.state.saturating_sub(PICK_SPEED),
                Direction::Down => pin.state.saturating_add(PICK_SPEED),
            };
            pin.state = cmp::min(pin.state, PIN_HEIGHT as u8 - pin.height);

            if pin.state == 0 {
                pin.direction = Direction::Down;
            }

            if (pin.state + pin.height) as u32 >= PIN_HEIGHT {
                pin.direction = Direction::Up;
            }
        }
    }

    pub fn button_action(&mut self) {
        if self.open == true {
            return;
        }

        let pin = self.current_pin();
        if !pin.is_near_shear() {
            self.current_pin += 1;
            return;
        }

        if self.current_pin == 0 {
            self.score += self.reward;
            self.open = true;
        } else {
            self.current_pin = self.current_pin.saturating_sub(1);
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
                Size::new(PIN_WIDTH - 2, pin.height as u32),
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
        gfx::render_tacos(display, self.score);
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
