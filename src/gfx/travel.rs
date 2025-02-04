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
const MAP_POINT: Point = Point::new(
    gfx::centered(gfx::DISPLAY_WIDTH, CELL_SIZE * MAP_X as u32),
    gfx::centered(gfx::DISPLAY_HEIGHT, CELL_SIZE * MAP_Y as u32),
);

const CELL_SIZE: u32 = 5;
const SUB_CELL_SIZE: u32 = 2;
static_assertions::const_assert!(CELL_SIZE == SUB_CELL_SIZE * 2 + 1);

// map
const MAP_X: usize = 15;
const MAP_Y: usize = 10;

pub struct Map([[bool; MAP_X]; MAP_Y]);

impl Map {
    fn above(&self, x: usize, y: usize) -> bool {
        let Some(y) = y.checked_sub(1) else {
            return false;
        };
        self.0[y][x]
    }

    fn below(&self, x: usize, y: usize) -> bool {
        let Some(row) = self.0.get(y + 1) else {
            return false;
        };
        row[x]
    }

    fn left(&self, x: usize, y: usize) -> bool {
        let Some(x) = x.checked_sub(1) else {
            return false;
        };
        self.0[y][x]
    }

    fn right(&self, x: usize, y: usize) -> bool {
        let Some(cell) = self.0[y].get(x + 1) else {
            return false;
        };
        *cell
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
pub const NUM_LANES: usize = 3;
const MIDDLE_STRIP_LENGTH: u8 = 5;
const LANE_HEIGHT: u32 = 18;
const BIKE_HEIGHT: u32 = 14;
const BIKE_WIDTH: u32 = 24;
const BIKE_Y_OFFSET: u32 = 3;
const FIRST_LANE_TOP_OFFSET: i32 =
    gfx::DISPLAY_HEIGHT - (LANE_HEIGHT as i32 + 1) * NUM_LANES as i32;
const BIKE_LEFT_OFFSET: i32 = 15;

enum LineOrientation {
    Horizontal,
    Vertical,
}

pub struct TravelState {
    pub score: u32,
    pub lanes: [u8; NUM_LANES],
    pub active_lane: u8,
    pub middle_strip: u8,
}

impl TravelState {
    pub fn new() -> Self {
        Self {
            score: 1338,
            lanes: [64, 128, 32],
            active_lane: 1,
            middle_strip: 0,
        }
    }

    pub fn tick(&mut self) {
        self.middle_strip += 1;
        self.middle_strip %= MIDDLE_STRIP_LENGTH;
    }

    pub fn draw_big_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        // lane rendering
        for (num, lane) in self.lanes.iter().enumerate() {
            let lane_point = Point::new(
                0,
                FIRST_LANE_TOP_OFFSET + ((LANE_HEIGHT as i32 + 1) * num as i32),
            );
            Rectangle::new(lane_point, Size::new(gfx::DISPLAY_WIDTH as u32, 1))
                .into_styled(gfx::WHITE)
                .draw(display)
                .unwrap();

            if num as u8 == self.active_lane {
                Rectangle::new(
                    lane_point + Point::new(BIKE_LEFT_OFFSET, BIKE_Y_OFFSET as i32),
                    Size::new(BIKE_WIDTH, BIKE_HEIGHT),
                )
                .into_styled(gfx::WHITE)
                .draw(display)
                .unwrap();
            }
        }

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

                // render lines
                if MAP.above(x, y) {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        SUB_CELL_SIZE as i32,
                        0,
                        LineOrientation::Vertical,
                    );
                }

                if MAP.below(x, y) {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        SUB_CELL_SIZE as i32,
                        SUB_CELL_SIZE as i32 + 1,
                        LineOrientation::Vertical,
                    );
                }

                if MAP.left(x, y) {
                    Self::draw_cell_line(
                        display,
                        cell_point,
                        0,
                        SUB_CELL_SIZE as i32,
                        LineOrientation::Horizontal,
                    );
                }

                if MAP.right(x, y) {
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
