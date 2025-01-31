use embedded_hal::i2c::I2c;
use ssd1306::mode::BufferedGraphicsMode;
pub use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

pub fn init<T: I2c>(
    i2c: T,
) -> Ssd1306<I2CInterface<T>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>> {
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    display
}
