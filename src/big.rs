use eh0::blocking::i2c;
use sh1106::{prelude::*, Builder};

pub fn init<T: i2c::Write>(i2c: T) -> GraphicsMode<I2cInterface<T>> {
    let mut display: GraphicsMode<_> = Builder::new()
        .with_rotation(DisplayRotation::Rotate180)
        .connect_i2c(i2c)
        .into();
    display.init().ok();
    display
}
