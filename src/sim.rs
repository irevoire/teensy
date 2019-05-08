use bit_field::BitField;
use volatile::Volatile;

/// doc/teensy_3.2.pdf - Page 235
/// System Integration Module (SIM)

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

/// doc/teensy_3.2.pdf - Page 236
/// SIM memory map
#[repr(C, packed)]
pub struct Sim {
    sopt1: Volatile<u32>,
    sopt1_cfg: Volatile<u32>,
    _pad0: Volatile<[u32; 1023]>,
    sopt2: Volatile<u32>,
    _pad1: Volatile<u32>,
    sopt4: Volatile<u32>,
    sopt5: Volatile<u32>,
    _pad2: Volatile<u32>,
    sopt7: Volatile<u32>,
    _pad3: Volatile<[u32; 2]>,
    sdid: Volatile<u32>,
    _pad4: Volatile<[u32; 3]>,
    scgc4: Volatile<u32>,
    scgc5: Volatile<u32>, /* clock gating control registers  */
    scgc6: Volatile<u32>,
    scgc7: Volatile<u32>,
    clkdiv1: Volatile<u32>,
    clkviv2: Volatile<u32>,
    fcfg1: Volatile<u32>,
    fcfg2: Volatile<u32>,
    uidh: Volatile<u32>,
    uidmh: Volatile<u32>,
    uidml: Volatile<u32>,
    uidl: Volatile<u32>,
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        &mut *(0x40047000 as *mut Sim)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    self.scgc5.update(|scgc5| {
                        scgc5.set_bit(11, true);
                    });
                }
            }
        }
    }

    pub fn set_dividers(&mut self, core: u32, bus: u32, flash: u32) {
        let mut clkdiv: u32 = 0;
        clkdiv.set_bits(28..32, core - 1);
        clkdiv.set_bits(24..28, bus - 1);
        clkdiv.set_bits(16..20, flash - 1);
        unsafe {
            self.clkdiv1.write(clkdiv);
        }
    }
}
