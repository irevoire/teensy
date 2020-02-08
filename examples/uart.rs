#![feature(stdsimd)]
#![no_std]
#![no_main]

use core::fmt::Write;
use teensy::*;

define_panic! {empty}

#[no_mangle]
fn main() {
    let mut time = core::time::Duration::new(0, 0);

    let (mut led, sim, uart) = unsafe {
        (
            make_pin!(led).make_gpio().with_output(),
            sim::Sim::new(),
            uart::UART::new(uart::UART0),
        )
    };
    unsafe {
        uart.setup(sim, 115200);
    }

    loop {
        led.toggle();
        sleep::sleep_ms(500);
        writeln!(uart, "Hello World: {:?}", time).unwrap(); // uart write can’t send error
        time += core::time::Duration::from_millis(500);
    }
}
