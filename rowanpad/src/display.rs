use embassy_rp::{
    i2c::{self, Async},
    peripherals::I2C1,
};
use embassy_time::Timer;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use rmk::{event::KeyEvent, keyboard::KEY_EVENT_CHANNEL};
use ssd1306::{
    mode::DisplayConfig, prelude::DisplayRotation, size::DisplaySize128x32, I2CDisplayInterface,
    Ssd1306,
};

pub async fn get_key_event() -> KeyEvent {
    return KEY_EVENT_CHANNEL.receive().await;
}

#[embassy_executor::task]
pub async fn run_display(i2c1: i2c::I2c<'static, I2C1, Async>) {
    let interface = I2CDisplayInterface::new(i2c1);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let mut battery = 0;
    let screen_size = display.size();
    loop {
        display.clear(BinaryColor::Off).unwrap();
        display.flush().unwrap();
        battery += 5;
        if battery > 100 {
            battery = 0;
        }

        Rectangle::new(
            Point::new(0, 0),
            Size::new(screen_size.width / 2, screen_size.height / 2),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();
        Timer::after_millis(400).await;
    }
}
