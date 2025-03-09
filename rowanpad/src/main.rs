#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;
#[macro_use]
mod display;
mod vial;

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Config};
use embassy_rp::peripherals::{I2C0, I2C1};
use embassy_rp::{
    bind_interrupts,
    flash::{Async, Flash},
    gpio::{AnyPin, Input, Output},
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
// use embassy_rp::flash::Blocking;
use panic_probe as _;
use rmk::{
    config::{KeyboardUsbConfig, RmkConfig, VialConfig},
    run_rmk_with_async_flash,
};
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

bind_interrupts!(struct I2CIrqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Pin config
    let (input_pins, output_pins) =
        config_matrix_pins_rp!(peripherals: p, input: [PIN_26, PIN_27], output: [PIN_2, PIN_4]);

    // Use internal flash to emulate eeprom
    // Both blocking and async flash are support, use different API
    // let flash = Flash::<_, Blocking, FLASH_SIZE>::new_blocking(p.FLASH);
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0);

    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "Haobo",
        product_name: "RMK Keyboard",
        serial_number: "vial:f64c2b3c:000001",
    };

    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);

    let rmk_config = RmkConfig {
        usb_config: keyboard_usb_config,
        vial_config,
        ..Default::default()
    };

    let mut i2c_config = Config::default();
    i2c_config.frequency = 400_000;

    spawner
        .spawn(display::run_display(i2c::I2c::new_async(
            p.I2C1, p.PIN_7, p.PIN_6, I2CIrqs, i2c_config,
        )))
        .unwrap();

    // Start serving
    // Use `run_rmk` for blocking flash
    run_rmk_with_async_flash(
        input_pins,
        output_pins,
        driver,
        flash,
        &mut keymap::get_default_keymap(),
        rmk_config,
        spawner,
    )
    .await;
}
