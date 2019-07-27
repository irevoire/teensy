#![allow(dead_code)]
#![feature(stdsimd)]
#![no_std]

pub mod boot;
pub mod interrupts;
pub mod mcg;
pub mod osc;
pub mod port;
pub mod sim;
pub mod watchdog;

extern "Rust" {
    fn main();
}
