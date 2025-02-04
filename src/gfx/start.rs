use crate::game::Screen;
use crate::gfx;
use core::fmt::Debug;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

// wait for everything to fully setup before taking inputs
const COOLDOWN: u8 = 2;

pub const BIG_TEXT: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_8X13)
    .text_color(BinaryColor::On)
    .build();

pub struct Start {
    cooldown: u8,
    pub transition: Option<Screen>,
}

impl Start {
    pub fn new() -> Self {
        Self {
            cooldown: COOLDOWN,
            transition: None,
        }
    }

    pub fn tick(&mut self) {
        self.cooldown = self.cooldown.saturating_sub(1);
    }

    pub fn button_action(&mut self) {
        if self.cooldown == 0 {
            self.transition = Some(Screen::Travel);
        }
    }

    pub fn draw_big_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        for (text, style, y) in [
            ("Taco Burglar", BIG_TEXT, 15),
            (".: Happy Birthday Ria :.", gfx::TEXT_STYLE, 43),
            ("2025", gfx::TEXT_STYLE, 50),
        ] {
            Text::with_baseline(
                text,
                Point::new(
                    gfx::text_align_center(text, gfx::DISPLAY_WIDTH, style.font),
                    y,
                ),
                style,
                Baseline::Top,
            )
            .draw(display)
            .unwrap();
        }
    }

    pub fn draw_small_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D)
    where
        <D as DrawTarget>::Error: Debug,
    {
        for (num, text) in [
            "Left buttons for up/down",
            "Upper lane to turn left",
            "Lower lane to turn right",
            "",
            "Red button to start game",
            "",
            "Be quick",
        ]
        .iter()
        .enumerate()
        {
            let num = num as i32;
            let y = num * (gfx::TEXT_STYLE.font.character_size.height + 1) as i32;
            Text::with_baseline(text, Point::new(0, y), gfx::TEXT_STYLE, Baseline::Top)
                .draw(display)
                .unwrap();
        }
    }
}
