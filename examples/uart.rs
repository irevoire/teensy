#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic! {empty}

#[no_mangle]
fn main() {
    let (led, sim, uart) = unsafe {
        (
            make_pin!(led),
            sim::Sim::new(),
            uart::UART_MemoryMap::new(uart::Available_UART::UART0),
        )
    };
    unsafe {
        uart.setup(sim, 115200);
    }

    let mut led = led.make_gpio();
    led.output();

    loop {
        led.toggle();
        uart.putchar('a');
        sleep::sleep_ms(500);
    }
}
