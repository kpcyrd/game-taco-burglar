#![warn(clippy::missing_const_for_fn)]
#![no_std]
#![no_main]

mod big;
mod game;
mod gfx;
mod i10n;
mod small;

use crate::game::Game;
use defmt_rtt as _;
use eh0::timer::CountDown;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_hal::digital::InputPin;
use fugit::ExtU32;
use fugit::RateExtU32;
use panic_halt as _;
use waveshare_rp2040_zero::entry;
use waveshare_rp2040_zero::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        i2c::I2C,
        pac,
        rosc::RingOscillator,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // Configure clocks and timers
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay = timer.count_down();
    let mut rosc = RingOscillator::new(pac.ROSC).initialize();

    // Configure gpio
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // configure button
    let mut action_in_pin = pins.gp8.into_pull_up_input();
    let mut up_in_pin = pins.gp27.into_pull_up_input();
    let mut down_in_pin = pins.gp15.into_pull_up_input();

    // setup i2c
    let small_i2c = I2C::i2c0(
        pac.I2C0,
        pins.gp12.into_pull_type().into_function(), // sda
        pins.gp13.into_pull_type().into_function(), // scl
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );
    let big_i2c = I2C::i2c1(
        pac.I2C1,
        pins.gp10.into_pull_type().into_function(), // sda
        pins.gp11.into_pull_type().into_function(), // scl
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );

    // init screens
    let mut small_display = small::init(small_i2c);
    let mut big_display = big::init(big_i2c);

    // game state
    let mut last_state = false;

    // enter loop
    let mut game = Game::new(&mut rosc);
    loop {
        while action_in_pin.is_low().unwrap() {
            if !last_state {
                game.button_action();
                last_state = true;
            }
            delay.start(50.millis());
            let _ = nb::block!(delay.wait());
        }
        last_state = false;

        while up_in_pin.is_low().unwrap() {
            if !last_state {
                game.button_up();
                last_state = true;
            }
            delay.start(50.millis());
            let _ = nb::block!(delay.wait());
        }
        last_state = false;

        while down_in_pin.is_low().unwrap() {
            if !last_state {
                game.button_down();
                last_state = true;
            }
            delay.start(50.millis());
            let _ = nb::block!(delay.wait());
        }
        last_state = false;

        // clear screens
        small_display.clear(BinaryColor::Off).unwrap();
        big_display.clear();

        // render small screen
        game.draw_small_screen(&mut small_display);
        small_display.flush().unwrap();

        // render big screen
        game.draw_big_screen(&mut big_display);
        big_display.flush().unwrap();

        // sleep for frame rate
        delay.start(50.millis());
        let _ = nb::block!(delay.wait());

        // process the concept of tick
        game.tick();
        game.transition();
    }
}
