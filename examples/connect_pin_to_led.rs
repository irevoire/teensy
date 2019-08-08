#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic! {empty}

#[no_mangle]
fn main() {
    let (led, mut pin) = unsafe { make_pin!(led, 1) };

    unsafe {
        pin.set_pin_pe(true);
        pin.set_pin_ps(false);
    }

    let mut led_out = led.make_gpio();
    let mut pin_in = pin.make_gpio();

    led_out.output();
    pin_in.input();

    loop {
        match pin_in.read() {
            0 => led_out.low(),
            _ => led_out.high(),
        }
    }
}
