//! At the moment this file is a rust reimplementation of this file:
//! https://github.com/mensi/teensy_bare_metal/blob/master/common/delay.c

use core::arch::arm::__nop;

/// This implementation will be more imprecise with lower CPU frequencies and
/// not properly work at all with anything below 5 MHz
pub fn sleep_us(microseconds: u32) {
    (0..microseconds).for_each(|_| {
        let mut inner = crate::mcg::F_CPU / 5000000;

        // This loop should take 5 cycles
        while inner != 0 {
            inner -= 1;
            unsafe {
                __nop();
            }
        }
    });
}

/// For milliseconds, this loop approach is not too far off reality as long
/// as it runs uninterrupted.
pub fn sleep_ms(milliseconds: u32) {
    (0..milliseconds).for_each(|_| {
        let mut inner = crate::mcg::F_CPU / 10000;

        // This loop should take 10 cycles:
        //  inner -= 1     sub: 1 cycle
        //  inner != 0     beq: 1 cycle if not taken
        //  6*nop          nop: 1 cycle
        //  endwhile       b:   1 + (1-3) cycles
        while inner != 0 {
            inner -= 1;
            unsafe {
                __nop(); // 1
                __nop(); // 2
                __nop(); // 3
                __nop(); // 4
                __nop(); // 5
                __nop(); // 6
            }
        }
    });
}
