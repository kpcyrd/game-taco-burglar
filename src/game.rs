use crate::gfx;
use core::fmt::Debug;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use rand_core::RngCore;

pub enum Screen {
    Start,
    Travel,
    Lock,
}

pub struct Game<R: RngCore> {
    random: R,
    screen: Screen,
    start: gfx::start::Start,
    travel: gfx::travel::TravelState,
    lock: gfx::lock::LockState,
}

impl<R: RngCore> Game<R> {
    pub fn new(mut random: R) -> Self {
        let start = gfx::start::Start::new();
        let travel = gfx::travel::TravelState::new(&mut random);
        let lock = gfx::lock::LockState::new();

        Self {
            random,
            start,
            screen: Screen::Start,
            travel,
            lock,
        }
    }

    // checks if the game state wants to transition to a different screen
    pub fn transition(&mut self) {
        match self.screen {
            Screen::Start => {
                if let Some(screen) = self.start.transition.take() {
                    // this is always travel state
                    self.travel = gfx::travel::TravelState::new(&mut self.random);
                    self.screen = screen;
                }
            }
            Screen::Travel => {
                let Some(screen) = self.travel.transition.take() else {
                    return;
                };

                match screen {
                    // game over
                    Screen::Start => self.screen = screen,
                    // not possible
                    Screen::Travel => (),
                    // switch to lock mini game
                    Screen::Lock => {
                        self.lock = gfx::lock::LockState::new();
                        self.lock.score = self.travel.score;
                        self.screen = screen;
                    }
                }
            }
            Screen::Lock => {}
        }
    }

    pub fn tick(&mut self) {
        match self.screen {
            Screen::Start => self.start.tick(),
            Screen::Travel => self.travel.tick(&mut self.random),
            Screen::Lock => (),
        }
    }

    pub fn button_action(&mut self) {
        match self.screen {
            Screen::Start => self.start.button_action(),
            Screen::Travel => (),
            Screen::Lock => (),
        }
    }

    pub fn button_up(&mut self) {
        match self.screen {
            Screen::Start => (),
            Screen::Travel => self.travel.button_up(),
            Screen::Lock => (),
        }
    }

    pub fn button_down(&mut self) {
        match self.screen {
            Screen::Start => (),
            Screen::Travel => self.travel.button_down(),
            Screen::Lock => (),
        }
    }

    pub fn draw_big_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        match self.screen {
            Screen::Start => self.start.draw_big_screen(display),
            Screen::Travel => self.travel.draw_big_screen(display),
            Screen::Lock => self.lock.draw_big_screen(display),
        }
    }

    pub fn draw_small_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        match self.screen {
            Screen::Start => self.start.draw_small_screen(display),
            Screen::Travel => self.travel.draw_small_screen(display),
            Screen::Lock => self.lock.draw_small_screen(display),
        }
    }
}
