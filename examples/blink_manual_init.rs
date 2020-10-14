#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic! {empty}

#[no_mangle]
fn init() {
    let (wdog, sim) = unsafe { (watchdog::Watchdog::new(), sim::Sim::new()) };

    wdog.disable();

    // Turn on all the port clock gate
    sim.enable_clock(sim::Clock::PortC);
}

#[no_mangle]
fn main() {
    let mut led = unsafe { make_pin!(led).make_gpio().output() };

    loop {
        led.toggle();
        sleep::sleep_ms(500);
    }
}
