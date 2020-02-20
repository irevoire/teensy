use core::arch::arm::__nop;

/// This function come directly from the [cortex_m
/// crate](https://docs.rs/cortex-m/0.6.2/cortex_m/asm/fn.delay.html)
/// It provide a delay in number of instructions. Use it only if you know what
/// you are doing.
#[inline]
pub fn delay(n: u64) {
    unsafe {
        asm!("1:
                  nop
                  subs $0, $$1
                  bne.n 1b"
                 : "+r"(n / 4 + 1)
                 :
                 :
                 : "volatile");
    }
}

/// This implementation will be more imprecise with lower CPU frequencies and
/// not properly work at all with anything below 5 MHz
#[inline]
pub fn sleep_us(microseconds: u32) {
    (0..microseconds).for_each(|_| {
        let mut inner = crate::mcg::F_CPU / 5_000_000;

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
        let mut inner = crate::mcg::F_CPU / 10_000;

        // This loop should take 10 cycles:
        //  inner -= 1     sub: 1 cycle
        //  inner != 0     beq: 1 cycle if not taken
        //  7*nop          nop: 1 cycle
        //  endwhile       b:   1 + (1-3) cycles
        while inner != 0 {
            inner -= 1;
            {
                __nop(); // 1
                __nop(); // 2
                __nop(); // 3
                __nop(); // 4
                __nop(); // 5
                __nop(); // 6
                __nop(); // 7
            }
        }
    });
}
