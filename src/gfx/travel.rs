use crate::game::Screen;
use crate::gfx;
use core::cmp;
use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
    text::{Baseline, Text},
};
use rand_core::RngCore;

const BIKE: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../../video/bike.raw"), 24);

// small screen consts
const MAP_POINT: Point = Point::new(
    gfx::centered(gfx::DISPLAY_WIDTH, CELL_SIZE * MAP_X as u32),
    gfx::centered(gfx::DISPLAY_HEIGHT, CELL_SIZE * MAP_Y as u32),
);

const TICKS_PER_TRAVEL_SQUARE: u8 = 7;

const CELL_SIZE: u32 = 5;
const SUB_CELL_SIZE: u32 = 2;
static_assertions::const_assert!(CELL_SIZE == SUB_CELL_SIZE * 2 + 1);

// map
const MAP_X: usize = 15;
const MAP_Y: usize = 10;

pub struct Map([[bool; MAP_X]; MAP_Y]);

impl Map {
    fn get(&self, x: usize, y: usize) -> bool {
        let Some(row) = self.0.get(y) else {
            return false;
        };
        let Some(cell) = row.get(x) else {
            return false;
        };
        *cell
    }

    fn above(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let y = y.checked_sub(1)?;
        self.0[y][x].then_some((x, y))
    }

    fn below(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let y = y + 1;
        let row = self.0.get(y)?;
        row[x].then_some((x, y))
    }

    fn left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let x = x.checked_sub(1)?;
        self.0[y][x].then_some((x, y))
    }

    fn right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let x = x + 1;
        let cell = self.0[y].get(x)?;
        cell.then_some((x, y))
    }

    fn direction(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::North => MAP.above(x, y),
            Direction::East => MAP.right(x, y),
            Direction::South => MAP.below(x, y),
            Direction::West => MAP.left(x, y),
        }
    }
}

const X: bool = true;
const O: bool = false;
const MAP: Map = Map([
    [X, X, X, X, X, X, X, X, X, X, X, X, X, O, O],
    [X, O, O, O, X, O, X, O, O, X, O, O, X, O, O],
    [X, O, X, O, X, X, X, X, X, X, X, X, X, O, O],
    [X, X, X, O, O, X, O, O, O, O, O, X, O, O, O],
    [O, O, O, O, O, X, X, X, X, X, X, X, X, X, X],
    [O, X, X, X, X, X, O, O, O, X, O, O, X, O, X],
    [O, X, O, O, O, X, O, O, O, X, O, X, X, O, X],
    [O, X, X, X, O, X, O, O, O, X, O, O, O, O, X],
    [O, X, O, O, O, X, X, X, X, X, O, O, O, O, X],
    [O, X, X, X, X, X, O, O, O, X, X, X, X, X, X],
]);

// big screen consts
pub const NUM_LANES: u8 = 3;
const MIDDLE_STRIP_LENGTH: u8 = 5;
const MIDDLE_STRIP_GAP: u8 = 10;
const MIDDLE_STRIP_STEP_SIZE: u8 = 3;
const LANE_HEIGHT: u32 = 18;

const BIKE_Y_OFFSET: u32 = 3;
const FIRST_LANE_TOP_OFFSET: i32 =
    gfx::DISPLAY_HEIGHT - (LANE_HEIGHT as i32 + 1) * NUM_LANES as i32;
const SECOND_LANE_TOP_OFFSET: i32 = FIRST_LANE_TOP_OFFSET + (LANE_HEIGHT as i32 + 1);
const THIRD_LANE_TOP_OFFSET: i32 = SECOND_LANE_TOP_OFFSET + (LANE_HEIGHT as i32 + 1);
const BIKE_LEFT_OFFSET: i32 = 13;

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const fn turn_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    const fn turn_counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    const fn as_str(&self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
        }
    }
}

enum LineOrientation {
    Horizontal,
    Vertical,
}

fn random_valid_position<R: RngCore>(mut random: R) -> (usize, usize) {
    loop {
        let num = random.next_u32() as usize % (MAP_X * MAP_Y);
        let y = num / MAP_X;
        let x = num - (y * MAP_X);
        if MAP.get(x, y) {
            return (x, y);
        }
    }
}

pub struct TravelState {
    pub score: u32,
    goal: (usize, usize),
    player: (usize, usize),
    direction: Direction,
    next_square: u8,
    active_lane: u8,
    middle_strip: u8,
    pub transition: Option<Screen>,
}

impl TravelState {
    pub fn new<R: RngCore>(mut random: R) -> Self {
        let mut state = Self {
            score: 0,
            goal: (0, 0),
            player: (0, 0),
            direction: Direction::North,
            next_square: TICKS_PER_TRAVEL_SQUARE,
            active_lane: 1,
            middle_strip: 0,
            transition: None,
        };
        state.set_random_player(&mut random);
        state.set_random_goal(&mut random);
        state
    }

    pub fn set_random_player<R: RngCore>(&mut self, random: R) {
        self.player = random_valid_position(random);
    }

    pub fn set_random_goal<R: RngCore>(&mut self, mut random: R) {
        loop {
            self.goal = random_valid_position(&mut random);
            // we may have to get a new value if player is already there
            if self.goal != self.player {
                break;
            }
        }
    }

    // try to turn in the selected direction, if possible
    fn try_turn(&mut self, new_direction: Direction) {
        let (x, y) = self.player;
        if MAP.direction(x, y, new_direction).is_some() {
            self.direction = new_direction;
        }
    }

    fn drive(&mut self) {
        loop {
            let (x, y) = self.player;
            // check if we can drive that way
            if let Some(pos) = MAP.direction(x, y, self.direction) {
                self.player = pos;
                break;
            }

            // the loop didn't break, check if we can do a clockwise turn
            let new_direction = self.direction.turn_clockwise();
            if MAP.direction(x, y, new_direction).is_some() {
                self.direction = new_direction;
                break;
            }

            // else, check if we can do a counter clockwise turn
            let new_direction = self.direction.turn_counter_clockwise();
            if MAP.direction(x, y, new_direction).is_some() {
                self.direction = new_direction;
                break;
            }

            // else, always do two clockwise to turn around
            self.direction = self.direction.turn_clockwise().turn_clockwise();
        }
    }

    pub fn button_up(&mut self) {
        self.active_lane = self.active_lane.saturating_sub(1);
    }

    pub fn button_down(&mut self) {
        self.active_lane = cmp::min(self.active_lane + 1, NUM_LANES - 1);
    }

    pub fn tick<R: RngCore>(&mut self, random: R) {
        // run animation
        self.middle_strip += MIDDLE_STRIP_STEP_SIZE;
        self.middle_strip %= MIDDLE_STRIP_LENGTH + MIDDLE_STRIP_GAP;

        // check if next square is reached
        self.next_square = self.next_square.saturating_sub(1);
        if self.next_square == 0 {
            // reset counter
            self.next_square = TICKS_PER_TRAVEL_SQUARE;

            // do turn
            self.try_turn(match self.active_lane {
                0 => self.direction.turn_counter_clockwise(),
                2 => self.direction.turn_clockwise(),
                _ => self.direction,
            });

            // drive in current direction
            self.drive();

            if self.player == self.goal {
                self.score += 100;
                // place new goal
                self.set_random_goal(random);
                // we want to switch to lock mini game
                self.transition = Some(Screen::Lock);
            }
        }
    }

    // render code

    pub fn draw_lane<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D, y: i32, full: bool)
    where
        <D as DrawTarget>::Error: Debug,
    {
        let lane_point = Point::new(0, y);

        if full {
            Rectangle::new(lane_point, Size::new(gfx::DISPLAY_WIDTH as u32, 1))
                .into_styled(gfx::WHITE)
                .draw(display)
                .unwrap();
        } else {
            let mut x = -(self.middle_strip as i32);
            while x < gfx::DISPLAY_WIDTH {
                // render current strip
                Rectangle::new(
                    lane_point + Point::new(x, 0),
                    Size::new(MIDDLE_STRIP_LENGTH as u32, 1),
                )
                .into_styled(gfx::WHITE)
                .draw(display)
                .unwrap();

                // add rendered strip
                x += MIDDLE_STRIP_LENGTH as i32;
                // gap
                x += MIDDLE_STRIP_GAP as i32;
            }
        }
    }

    pub fn draw_big_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        // render lanes
        self.draw_lane(display, FIRST_LANE_TOP_OFFSET, true);
        self.draw_lane(display, SECOND_LANE_TOP_OFFSET, false);
        self.draw_lane(display, THIRD_LANE_TOP_OFFSET, false);

        // render bike
        let bike_point = Point::new(
            BIKE_LEFT_OFFSET,
            match self.active_lane {
                0 => FIRST_LANE_TOP_OFFSET,
                1 => SECOND_LANE_TOP_OFFSET,
                _ => THIRD_LANE_TOP_OFFSET,
            } + BIKE_Y_OFFSET as i32,
        );
        Image::new(&BIKE, bike_point).draw(display).unwrap();

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

        // render direction
        Text::with_baseline(
            self.direction.as_str(),
            Point::new(0, 0),
            gfx::TEXT_STYLE,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }

    #[inline]
    fn draw_cell_line<D: DrawTarget<Color = BinaryColor>>(
        display: &mut D,
        cell_point: Point,
        pos_x: i32,
        pos_y: i32,
        orientation: LineOrientation,
    ) where
        <D as DrawTarget>::Error: Debug,
    {
        let size = match orientation {
            LineOrientation::Horizontal => Size::new(SUB_CELL_SIZE, 1),
            LineOrientation::Vertical => Size::new(1, SUB_CELL_SIZE),
        };

        Rectangle::new(cell_point + Point::new(pos_x, pos_y), size)
            .into_styled(gfx::WHITE)
            .draw(display)
            .unwrap();
    }

    pub fn draw_small_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        for (y, row) in MAP.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if !*cell {
                    continue;
                };

                let cell_point = MAP_POINT
                    + Point::new((x as u32 * CELL_SIZE) as i32, (y as u32 * CELL_SIZE) as i32);

                if (x, y) == self.goal || (x, y) == self.player {
                    // they both share this white rectangle
                    Rectangle::new(cell_point + Point::new(1, 1), Size::new(3, 3))
                        .into_styled(gfx::WHITE)
                        .draw(display)
                        .unwrap();
                    // goal has a black dot in the middle
                    if (x, y) == self.goal {
                        Rectangle::new(
                            cell_point + Point::new(SUB_CELL_SIZE as i32, SUB_CELL_SIZE as i32),
                            Size::new(1, 1),
                        )
                        .into_styled(gfx::BLACK)
                        .draw(display)
                        .unwrap();
                    }
                    continue;
                }

                // render lines
                if MAP.above(x, y).is_some() {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        SUB_CELL_SIZE as i32,
                        0,
                        LineOrientation::Vertical,
                    );
                }

                if MAP.below(x, y).is_some() {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        SUB_CELL_SIZE as i32,
                        SUB_CELL_SIZE as i32 + 1,
                        LineOrientation::Vertical,
                    );
                }

                if MAP.left(x, y).is_some() {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        0,
                        SUB_CELL_SIZE as i32,
                        LineOrientation::Horizontal,
                    );
                }

                if MAP.right(x, y).is_some() {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        SUB_CELL_SIZE as i32 + 1,
                        SUB_CELL_SIZE as i32,
                        LineOrientation::Horizontal,
                    );
                }

                // render center
                Rectangle::new(
                    cell_point + Point::new(SUB_CELL_SIZE as i32, SUB_CELL_SIZE as i32),
                    Size::new(1, 1),
                )
                .into_styled(gfx::WHITE)
                .draw(display)
                .unwrap();
            }
        }
    }
}
