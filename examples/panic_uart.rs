#![feature(stdsimd)]
#![no_std]
#![no_main]

teensy::define_panic! {uart}

#[no_mangle]
fn main() {
    panic!("Hello");
}
