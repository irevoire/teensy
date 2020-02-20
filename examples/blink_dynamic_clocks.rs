#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::mcg::{CpuFreq, Mcg};
use teensy::sim::Sim;
use teensy::*;

define_panic! {blink}

#[no_mangle]
fn main() {
    let mut led = unsafe { make_pin!(led).make_gpio().with_output() };

    loop {
        change_clocks(CpuFreq::High);
        led.toggle();
        sleep::delay(72_000_000 * 5); // 5s at 72MHz -> 3.75s at 96MHz
        led.toggle();
        sleep::delay(72_000_000 * 5); // 5s at 72MHz -> 3.75s at 96MHz

        change_clocks(CpuFreq::Reduced);
        led.toggle();
        sleep::delay(72_000_000 * 5); // 5s at 72MHz -> 7.5s at 48MHz
        led.toggle();
        sleep::delay(72_000_000 * 5); // 5s at 72MHz -> 7.5s at 48MHz
    }
}

fn change_clocks(freq: mcg::CpuFreq) {
    let (sim, mcg): (&mut Sim, &mut Mcg) = unsafe { (sim::Sim::new(), mcg::Mcg::new()) };
    mcg.set_clocks(freq, sim);
}
