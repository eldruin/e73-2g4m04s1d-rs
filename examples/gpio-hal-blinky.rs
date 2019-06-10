//! Blinks an LED connected to pin P0.20.
//!
//! In addition to the E73 board and the ST-Link V2 programmer you will need
//! a 220 Ohm resistence (or similar) and an LED.
//! The resistence will be put in between P0.20 and the LED.
//!
//! ```
//! ST-Link  <-> E73   <-> Resistence <-> LED
//! GND      <-> GND                  <-> GND
//! +3.3V    <-> VCC
//! SWDCLK   <-> SWDCLK
//! SWDIO    <-> SWDIO
//!              P0.20 <-> one end
//!                        other end  <-> VCC
//! ```
//!
//! Run with:
//! `cargo run --example gpio-hal-blinky`,

#![no_main]
#![no_std]

extern crate panic_halt;
use hal::delay::Delay;
use hal::gpio::GpioExt;
use hal::hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use nrf51_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = nrf51::Peripherals::take() {
        let gpio = p.GPIO.split();
        let mut delay = Delay::new(p.TIMER0);
        // change next line to use another pin
        let mut led = gpio.pin20.into_push_pull_output();

        loop {
            led.set_low().unwrap();
            delay.delay_ms(1000_u16); // 1 second
            led.set_high().unwrap();
            delay.delay_ms(1000_u16); // 1 second
        }
    }

    loop {
        continue;
    }
}
