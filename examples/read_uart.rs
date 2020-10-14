#![feature(stdsimd)]
#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_hal::prelude::*;
use teensy::*;

define_panic! {empty}

#[no_mangle]
fn main() {
    let (led, sim, uart) = unsafe {
        (
            make_pin!(led),
            sim::Sim::new(),
            uart::UART::new(uart::UART0),
        )
    };
    unsafe {
        uart.setup(sim, 115200);
    }

    let mut led = led.make_gpio().output();

    let mut start = 0;
    let mut buf = ['a'; 30];
    loop {
        writeln!(uart, "Waiting for input: {}", start).unwrap();
        start += 1;
        led.try_toggle().unwrap();
        sleep::sleep_ms(500);
        uart.read_line(&mut buf);
        write!(uart, "got: {:?}", buf).unwrap();
    }
}
