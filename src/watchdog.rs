//! # Watchdog
//! The watchdog is a piece of hardware which will reset the microcontroller unless the running
//! application “checks in” in a certain interval. It’s designed to restart crashed or hung
//! programs. Currently we can only disable it.

use bit_field::BitField;
use core::arch::arm::__nop;
use volatile::Volatile;

#[repr(C, packed)]
pub struct Watchdog {
    stctrlh: Volatile<u16>,
    stctrll: Volatile<u16>,
    tovalh: Volatile<u16>,
    tovall: Volatile<u16>,
    winh: Volatile<u16>,
    winl: Volatile<u16>,
    refresh: Volatile<u16>,
    unlock: Volatile<u16>,
    tmrouth: Volatile<u16>,
    tmroutl: Volatile<u16>,
    rstcnt: Volatile<u16>,
    presc: Volatile<u16>,
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x4005_2000 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            self.unlock.write(0xC520);
            self.unlock.write(0xD928);
            __nop();
            __nop();
            self.stctrlh.update(|ctrl| {
                ctrl.set_bit(0, false);
            });
        }
    }
}
