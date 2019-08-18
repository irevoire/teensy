#![feature(stdsimd)]
#![no_std]
#![no_main]

teensy::define_panic! {blink}

#[no_mangle]
fn main() {
    let a = 40;
    let b = 2;
    let _c = (a + b) / (a - 20 * b);
    // 42 / 0 => panic
}
