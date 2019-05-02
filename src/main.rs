#![feature(stdsimd)]
#![no_std]
#![no_main]

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
extern fn main() {
    let (wdog,sim,pin) = unsafe {
        (watchdog::Watchdog::new(),
        sim::Sim::new(),
        port::Port::new(port::PortName::C).pin(5))
    };

    wdog.disable();
    sim.enable_clock(sim::Clock::PortC);

    let mut gpio = pin.make_gpio();

    gpio.output();
    gpio.low();

    gpio.high();
    loop {
        sleep();
        gpio.low();
        sleep();
    }
}

#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {};
}

extern {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern fn(); 2] = [
    _stack_top,
    main,
];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF
];
