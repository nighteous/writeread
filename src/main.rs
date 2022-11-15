#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use embedded_hal::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // led to check
    let mut led2 = pins.d2.into_output();
    let mut led3 = pins.d3.into_output();
    let mut led4 = pins.d4.into_output();
    let mut led5 = pins.d5.into_output();
    let mut led6 = pins.d6.into_output();

    let mut buzzer = pins.d12.into_output();

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        led2.set_low();
        led3.set_low();
        led4.set_low();
        led5.set_low();
        led6.set_low();

        buzzer.set_low();

        if b == 49 {
            led2.toggle();
        }
        else if b == 50 {
            led2.toggle();
            led3.toggle();
        }
        else if b == 51 {
            led2.toggle();
            led3.toggle();
            led4.toggle();
        }
        else if b == 52 {
            led2.toggle();
            led3.toggle();
            led4.toggle();
            led5.toggle();
        }
        else if b == 53 {
            led2.toggle();
            led3.toggle();
            led4.toggle();
            led5.toggle();
            led6.toggle();

            buzzer.toggle();
        }

        arduino_hal::delay_ms(500);
    }
}
