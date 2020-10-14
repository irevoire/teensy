#![feature(stdsimd)]
#![no_std]
#![no_main]

use embedded_hal::prelude::*;
use teensy::*;

define_panic! {empty}

#[no_mangle]
fn main() {
    let (led, mut pin) = unsafe { make_pin!(led, 6) };

    unsafe {
        pin.set_pin_pe(true);
        pin.set_pin_ps(false);
    }

    let mut led_out = led.make_gpio().output();
    let pin_in = pin.make_gpio().input();

    loop {
        match pin_in.try_is_high().unwrap() {
            false => led_out.try_set_low().unwrap(),
            true => led_out.try_set_high().unwrap(),
        }
    }
}
