use crate::game::Screen;
use crate::gfx;
use crate::i10n;
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
const ALIVENESS_MODULO: u8 = 4;
const ALIVENESS_SLOWDOWN: u8 = 3;

pub const BIG_TEXT: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_8X13)
    .text_color(BinaryColor::On)
    .build();

pub struct Start {
    cooldown: u8,
    aliveness: u8,
    pub transition: Option<Screen>,
}

impl Start {
    pub const fn new() -> Self {
        Self {
            cooldown: COOLDOWN,
            aliveness: 0,
            transition: None,
        }
    }

    pub fn tick(&mut self) {
        self.cooldown = self.cooldown.saturating_sub(1);
        self.aliveness = (self.aliveness + 1) % (ALIVENESS_MODULO * ALIVENESS_SLOWDOWN);
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
        for (num, text) in i10n::INSTRUCTIONS.iter().enumerate() {
            let text = text[(self.aliveness / ALIVENESS_SLOWDOWN) as usize % text.len()];
            let num = num as i32;
            let y = num * (gfx::TEXT_STYLE.font.character_size.height + 1) as i32;
            Text::with_baseline(text, Point::new(0, y), gfx::TEXT_STYLE, Baseline::Top)
                .draw(display)
                .unwrap();
        }
    }
}
