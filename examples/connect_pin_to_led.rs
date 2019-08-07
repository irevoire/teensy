#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;
use teensy::port::Pin;
use teensy::port::Port;

define_panic!{empty}

#[no_mangle]
fn main() {
    let (mut led, mut pin): (Pin, Pin) = unsafe { make_pin!(led, 3) };

    let mut led = led.make_gpio();
    let mut pin = pin.make_gpio();

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
