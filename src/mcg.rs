use core::mem;

use crate::sim;
use bit_field::BitField;
use volatile::Volatile;

pub static mut F_CPU: u32 = 72_000_000;

#[repr(C, packed)]
pub struct Mcg {
    c1: Volatile<u8>,
    c2: Volatile<u8>,
    c3: Volatile<u8>,
    c4: Volatile<u8>,
    c5: Volatile<u8>,
    c6: Volatile<u8>,
    s: Volatile<u8>,
    _pad0: u8,
    sc: Volatile<u8>,
    _pad1: u8,
    atcvh: Volatile<u8>,
    atcvl: Volatile<u8>,
    c7: Volatile<u8>,
    c8: Volatile<u8>,
}

impl Mcg {
    pub unsafe fn new() -> &'static mut Mcg {
        &mut *(0x4006_4000 as *mut Mcg)
    }
}

pub enum OscRange {
    Low = 0,
    High = 1,
    VeryHigh = 2,
}

enum OscSource {
    LockedLoop = 0,
    Internal = 1,
    External = 2,
}

pub enum CpuFreq {
    /// Core/Bus/Flash speeds
    High,    // 96/48/24 (96MHz PLL)
    Default, // 72/36/24 (72MHz PLL)
    Reduced, // 48/48/24 (96MHz PLL)
    Low,     // 24/24/24 (96MHz PLL)
}

pub struct Fei {
    mcg: &'static mut Mcg,
}

impl Fei {
    pub fn enable_xtal(&mut self, range: OscRange) {
        self.mcg.c2.update(|c2| {
            c2.set_bits(4..6, range as u8);
            c2.set_bit(2, true);
        });

        // Wait for the crystal oscillator to become enabled.
        while !self.mcg.s.read().get_bit(1) {}
    }

    pub fn use_external(self, divide: u32) -> Fbe {
        let osc = self.mcg.c2.read().get_bits(4..6);
        let frdiv = if osc == OscRange::Low as u8 {
            match divide {
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                16 => 4,
                32 => 5,
                64 => 6,
                128 => 7,
                _ => panic!("Invalid external clock divider: {}", divide),
            }
        } else {
            match divide {
                32 => 0,
                64 => 1,
                128 => 2,
                256 => 3,
                512 => 4,
                1024 => 5,
                1280 => 6,
                1536 => 7,
                _ => panic!("Invalid external clock divider: {}", divide),
            }
        };

        self.mcg.c1.update(|c1| {
            c1.set_bits(6..8, OscSource::External as u8);
            c1.set_bits(3..6, frdiv);
            c1.set_bit(2, false);
        });

        // Once we write to the control register, we need to wait for
        // the new clock to stabilize before we move on.
        // First: Wait for the FLL to be pointed at the crystal
        // Then: Wait for our clock source to be the crystal osc
        while self.mcg.s.read().get_bit(4) {}
        while self.mcg.s.read().get_bits(2..4) != OscSource::External as u8 {}

        Fbe { mcg: self.mcg }
    }
}

pub struct Fbe {
    mcg: &'static mut Mcg,
}

impl Fbe {
    // numerator / denominator * xtal (16MHz) = clock speed??
    // enable phase-locked loop
    pub fn enable_pll(self, numerator: u8, denominator: u8) -> Pbe {
        if numerator < 24 || numerator > 55 {
            panic!("Invalid PLL VCO divide factor: {}", numerator);
        }

        if denominator < 1 || denominator > 25 {
            panic!("Invalid PLL reference divide factor: {}", denominator);
        }

        self.mcg.c5.update(|c5| {
            // set PLL external reference divide factor
            c5.set_bits(0..5, denominator - 1);
            // Subtract 1 here to make math easier
        });

        self.mcg.c6.update(|c6| {
            // set VCO divider
            c6.set_bits(0..5, numerator - 24);
            // select PLL
            c6.set_bit(6, true);
        });

        // Wait for PLL to be enabled
        while !self.mcg.s.read().get_bit(5) {}
        // Wait for the PLL to be "locked" and stable
        while !self.mcg.s.read().get_bit(6) {}

        Pbe { mcg: self.mcg }
    }
}

pub struct Pbe {
    mcg: &'static mut Mcg,
}

impl Pbe {
    pub fn use_pll(self) -> Pee {
        self.mcg.c1.update(|c1| {
            c1.set_bits(6..8, OscSource::LockedLoop as u8);
        });

        // mcg.c1 and mcg.s have slightly different behaviors.  In c1,
        // we use one value to indicate "Use whichever LL is
        // enabled". In s, it is differentiated between the FLL at 0,
        // and the PLL at 3. Instead of adding a value to OscSource
        // which would be invalid to set, we just check for the known
        // value "3" here.
        while self.mcg.s.read().get_bits(2..4) != 3 {}
        Pee { mcg: self.mcg }
    }

    pub fn disable_pll(self) -> Fbe {
        self.mcg.c6.update(|c6| {
            // select FLL
            c6.set_bit(6, false);
        });

        // Wait for PLL to be disabled
        while self.mcg.s.read().get_bit(5) {}
        // Wait for the PLL to be "unlocked"
        while self.mcg.s.read().get_bit(6) {}

        Fbe { mcg: self.mcg }
    }
}

pub struct Pee {
    mcg: &'static mut Mcg,
}

impl Pee {
    fn use_external(self) -> Pbe {
        self.mcg.c1.update(|c1| {
            c1.set_bits(6..8, OscSource::External as u8);
        });

        // mcg.c1 and mcg.s have slightly different behaviors.  In c1,
        // we use one value to indicate "Use whichever LL is
        // enabled". In s, it is differentiated between the FLL at 0,
        // and the PLL at 3. Instead of adding a value to OscSource
        // which would be invalid to set, we just check for the known
        // value "0" here.
        while self.mcg.s.read().get_bits(2..4) != 2 {}
        return Pbe { mcg: self.mcg };
    }
}

pub enum Clock {
    Fei(Fei),
    Fbe(Fbe),
    Pbe(Pbe),
    Pee(Pee),
}

impl Mcg {
    pub fn clock(&'static mut self) -> Clock {
        let source: OscSource = unsafe { mem::transmute(self.c1.read().get_bits(6..8)) };
        let fll_internal = self.c1.read().get_bit(2);
        let pll_enabled = self.c6.read().get_bit(6);

        // TODO: match all possible MCG clock modes before panic
        match (fll_internal, pll_enabled, source) {
            (true, false, OscSource::LockedLoop) => Clock::Fei(Fei { mcg: self }),
            (false, false, OscSource::External) => Clock::Fbe(Fbe { mcg: self }),
            (_, true, OscSource::External) => Clock::Pbe(Pbe { mcg: self }),
            (_, true, OscSource::LockedLoop) => Clock::Pee(Pee { mcg: self }),
            _ => panic!("The current clock mode cannot be represented as a known struct"),
        }
    }

    pub fn set_clocks(&'static mut self, clock: CpuFreq, sim: &mut sim::Sim) -> Pee {
        match clock {
            CpuFreq::High => {
                unsafe {
                    F_CPU = 96_000_000;
                }
                // Set our clocks: 96/48/24
                sim.set_dividers(1, 2, 4);
                // We would also set the USB divider here if we wanted to use it.
                let fbe = self.mode_to_fbe();
                let pbe = fbe.enable_pll(24, 4); // 16MHz / 4 * 24 = 96MHz
                pbe.use_pll()
            }
            CpuFreq::Default => {
                unsafe {
                    F_CPU = 72_000_000;
                }
                // Set our clocks: 72/36/24
                sim.set_dividers(1, 2, 3);
                // We would also set the USB divider here if we wanted to use it.
                let fbe = self.mode_to_fbe();
                let pbe = fbe.enable_pll(27, 6); // 16MHz / 6 * 27 = 72MHz
                pbe.use_pll()
            }
            CpuFreq::Reduced => {
                unsafe {
                    F_CPU = 48_000_000;
                }
                // Set our clocks: 48/48/24
                sim.set_dividers(2, 2, 4);
                // We would also set the USB divider here if we wanted to use it.
                let fbe = self.mode_to_fbe();
                let pbe = fbe.enable_pll(24, 4); // 16MHz / 4 * 24 = 96MHz
                pbe.use_pll()
            }
            CpuFreq::Low => {
                unsafe {
                    F_CPU = 24_000_000;
                }
                // Set our clocks: 24/24/24
                sim.set_dividers(4, 4, 4);
                // We would also set the USB divider here if we wanted to use it.
                let fbe = self.mode_to_fbe();
                let pbe = fbe.enable_pll(24, 4); // 16MHz / 4 * 24 = 96MHz
                pbe.use_pll()
            }
        }
    }

    pub fn mode_to_fbe(&'static mut self) -> Fbe {
        return match self.clock() {
            Clock::Fei(mut fei) => {
                // Our 16MHz xtal is "very fast", and needs to be divided
                // by 512 to be in the acceptable FLL range.
                // 31.25 kHz to 39.0625 kHz
                fei.enable_xtal(OscRange::VeryHigh);
                // (literally the only valid divisor on teensy)
                fei.use_external(512) // 16MHz/512 = 31.25KHz
            }
            Clock::Fbe(fbe) => fbe,
            Clock::Pbe(pbe) => pbe.disable_pll(),
            Clock::Pee(pee) => {
                let pbe = pee.use_external();
                let fbe = pbe.disable_pll();
                return fbe;
            }
        };
    }
}
