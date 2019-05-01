#![feature(stdsimd)]
#![no_std]
#![no_main]

mod port;
mod sim;
mod watchdog;

#[no_mangle]
fn sleep() -> u32 {
    let mut a = 0;
    for _ in 0..200000 {
        unsafe {
            let mut pcr = core::ptr::read_volatile(&a);
        }
        a += 1;
    }
    a
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
    let mut a = 0;
    loop {
    gpio.output();
        a -= sleep();
        gpio.low();
        a += sleep();
        if a == 2 {
            break;
        }
    }
    unsafe {
        core::ptr::write_volatile(&mut 42, a);
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
