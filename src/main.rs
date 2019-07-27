#![allow(dead_code)]
#![feature(stdsimd)]
#![no_std]
#![no_main]

mod boot;
mod mcg;
mod osc;
mod port;
mod sim;
mod watchdog;

#[no_mangle]
fn sleep() {
    for _ in 0..200000 {
        unsafe {
            core::arch::arm::__nop();
        }
    }
}

#[no_mangle]
extern "C" fn main() {
    let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };

    let mut gpio = pin.make_gpio();

    gpio.output();

    loop {
        sleep();
        sleep();
        sleep();
        gpio.toggle();
    }
}
