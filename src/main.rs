#![no_std]
#![no_main]

mod big;
mod small;

use defmt_rtt as _;
use eh0::timer::CountDown;
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
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
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

const FRAMES: &[ImageRaw<BinaryColor>] = &[
    ImageRaw::new(include_bytes!("../video/frame1.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame2.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame3.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame4.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame5.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame6.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame7.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame8.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame9.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame10.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame11.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame12.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame13.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame14.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame15.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame16.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame17.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame18.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame19.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame20.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame21.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame22.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame23.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame24.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame25.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame26.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame27.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame28.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame29.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame30.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame31.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame32.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame33.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame34.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame35.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame36.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame37.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame38.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame39.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame40.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame41.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame42.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame43.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame44.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame45.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame46.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame47.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame48.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame49.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame50.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame51.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame52.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame53.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame54.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame55.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame56.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame57.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame58.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame59.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame60.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame61.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame62.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame63.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame64.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame65.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame66.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame67.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame68.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame69.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame70.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame71.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame72.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame73.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame74.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame75.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame76.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame77.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame78.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame79.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame80.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame81.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame82.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame83.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame84.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame85.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame86.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame87.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame88.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame89.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame90.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame91.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame92.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame93.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame94.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame95.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame96.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame97.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame98.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame99.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame100.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame101.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame102.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame103.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame104.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame105.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame106.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame107.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame108.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame109.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame110.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame111.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame112.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame113.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame114.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame115.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame116.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame117.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame118.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame119.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame120.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame121.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame122.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame123.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame124.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame125.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame126.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame127.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame128.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame129.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame130.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame131.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame132.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame133.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame134.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame135.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame136.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame137.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame138.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame139.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame140.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame141.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame142.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame143.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame144.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame145.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame146.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame147.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame148.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame149.raw"), 128),
    ImageRaw::new(include_bytes!("../video/frame150.raw"), 128),
];

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

    // Configure small display
    let i2c = I2C::i2c0(
        pac.I2C0,
        pins.gp12.into_pull_type().into_function(), // sda
        pins.gp13.into_pull_type().into_function(), // scl
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );
    let mut small_display = small::init(i2c);

    // Configure big display
    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.gp10.into_pull_type().into_function(), // sda
        pins.gp11.into_pull_type().into_function(), // scl
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );
    let mut big_display = big::init(i2c);

    // game state
    let mut last_state = false;
    let mut ctr = 0;
    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    // enter loop
    let mut iter = [].iter();
    loop {
        // get next frame or restart iterator
        let Some(raw) = iter.next() else {
            iter = FRAMES.iter();
            continue;
        };

        while action_in_pin.is_low().unwrap() {
            if !last_state {
                ctr += 1;
                last_state = true;
            }
            delay.start(50.millis());
            let _ = nb::block!(delay.wait());
        }
        last_state = false;

        while up_in_pin.is_low().unwrap() {
            if !last_state {
                ctr = 100;
                last_state = true;
            }
            delay.start(50.millis());
            let _ = nb::block!(delay.wait());
        }
        last_state = false;

        while down_in_pin.is_low().unwrap() {
            if !last_state {
                ctr = 0;
                last_state = true;
            }
            delay.start(50.millis());
            let _ = nb::block!(delay.wait());
        }
        last_state = false;

        // clear screens
        small_display.clear(BinaryColor::Off).unwrap();
        big_display.clear();

        // draw image
        let im = Image::new(raw, Point::new(0, 13));
        im.draw(&mut small_display).unwrap();
        small_display.flush().unwrap();

        im.draw(&mut big_display).unwrap();
        let mut buf = itoa::Buffer::new();
        let buf = buf.format(ctr);
        Text::new(&buf, Point::new(10, 10), style)
            .draw(&mut big_display)
            .unwrap();
        big_display.flush().unwrap();

        // sleep for frame rate
        delay.start(50.millis());
        let _ = nb::block!(delay.wait());
    }
}
