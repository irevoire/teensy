use bit_field::BitField;
use volatile::Volatile;

const UART0_BASE_PTR: u32 = 0x4006_A000;
const UART1_BASE_PTR: u32 = 0x4006_B000;
const UART2_BASE_PTR: u32 = 0x4006_C000;
const UART3_BASE_PTR: u32 = 0x4006_D000;
const UART4_BASE_PTR: u32 = 0x400E_A000;

const UART_BDH_MASK: u8 = 0x1F;
const UART_BDL_MASK: u8 = 0xFF;
const UART_C4_BRFA_MASK: u8 = 0x1F;
const UART_C2_TX_ENABLE_MASK: u8 = 0x08;
const UART_C2_RX_ENABLE_MASK: u8 = 0x04;
const UART_S1_TDRE_MASK: u8 = 0x80;

#[allow(non_camel_case_types)]
pub enum Available_UART {
    UART0,
    UART1,
    UART2,
}

use Available_UART::*;

#[repr(C, packed)]
#[allow(non_snake_case)]
pub struct UART {
    /// < UART Baud Rate Registers:High, offset: 0x0
    BDH: Volatile<u8>,
    /// < UART Baud Rate Registers: Low, offset: 0x1
    BDL: Volatile<u8>,
    /// < UART Control Register 1, offset: 0x2
    C1: Volatile<u8>,
    /// < UART Control Register 2, offset: 0x3
    C2: Volatile<u8>,
    /// < UART Status Register 1, offset: 0x4
    S1: Volatile<u8>,
    /// < UART Status Register 2, offset: 0x5
    S2: Volatile<u8>,
    /// < UART Control Register 3, offset: 0x6
    C3: Volatile<u8>,
    /// < UART Data Register, offset: 0x7
    D: Volatile<u8>,
    /// < UART Match Address Registers 1, offset: 0x8
    MA1: Volatile<u8>,
    /// < UART Match Address Registers 2, offset: 0x9
    MA2: Volatile<u8>,
    /// < UART Control Register 4, offset: 0xA
    C4: Volatile<u8>,
    /// < UART Control Register 5, offset: 0xB
    C5: Volatile<u8>,
    /// < UART Extended Data Register, offset: 0xC
    ED: Volatile<u8>,
    /// < UART Modem Register, offset: 0xD
    MODEM: Volatile<u8>,
    /// < UART Infrared Register, offset: 0xE
    IR: Volatile<u8>,
    RESERVED: Volatile<u8>,
    /// < UART FIFO Parameters, offset: 0x10
    PFIFO: Volatile<u8>,
    /// < UART FIFO Control Register, offset: 0x11
    CFIFO: Volatile<u8>,
    /// < UART FIFO Status Register, offset: 0x12
    SFIFO: Volatile<u8>,
    /// < UART FIFO Transmit Watermark, offset: 0x13
    TWFIFO: Volatile<u8>,
    /// < UART FIFO Transmit Count, offset: 0x14
    TCFIFO: Volatile<u8>,
    /// < UART FIFO Receive Watermark, offset: 0x15
    RWFIFO: Volatile<u8>,
    /// < UART FIFO Receive Count, offset: 0x16
    RCFIFO: Volatile<u8>,
    RESERVED_1: Volatile<u8>,
    /// < UART 7816 Control Register, offset: 0x18
    C7816: Volatile<u8>,
    /// < UART 7816 Interrupt Enable Register, offset: 0x19
    IE7816: Volatile<u8>,
    /// < UART 7816 Interrupt Status Register, offset: 0x1A
    IS7816: Volatile<u8>,
    /// < UART 7816 Wait Parameter Register, offset: 0x1B
    WP7816_T_TYPE: Volatile<u8>,
    /// < UART 7816 Wait N Register, offset: 0x1C
    WN7816: Volatile<u8>,
    /// < UART 7816 Wait FD Register, offset: 0x1D
    WF7816: Volatile<u8>,
    /// < UART 7816 Error Threshold Register, offset: 0x1E
    ET7816: Volatile<u8>,
    /// < UART 7816 Transmit Length Register, offset: 0x1F
    TL7816: Volatile<u8>,
    // TODO add the end of the struct p 1206
}

impl UART {
    pub unsafe fn new(num: Available_UART) -> &'static mut UART {
        &mut *match num {
            UART0 => {
                let (mut rx, mut tx) = (crate::port::Pin::new(0), crate::port::Pin::new(1));
                rx.set_pin_mode(3);
                tx.set_pin_mode(3);
                UART0_BASE_PTR as *mut UART
            }
            UART1 => {
                let (mut rx, mut tx) = (crate::port::Pin::new(9), crate::port::Pin::new(10));
                rx.set_pin_mode(3);
                tx.set_pin_mode(3);
                UART1_BASE_PTR as *mut UART
            }
            UART2 => {
                let (mut rx, mut tx) = (crate::port::Pin::new(7), crate::port::Pin::new(8));
                rx.set_pin_mode(3);
                tx.set_pin_mode(3);
                UART2_BASE_PTR as *mut UART
            }
        }
    }
    pub fn name(&self) -> Available_UART {
        let addr = (self as *const UART) as u32;
        match addr {
            UART0_BASE_PTR => UART0,
            UART1_BASE_PTR => UART1,
            UART2_BASE_PTR => UART2,
            _ => unreachable!(),
        }
    }

    /// UART configuration is described p.1291 of doc/mx20dx256vhl7.pdf
    pub unsafe fn setup(&mut self, sim: &mut crate::sim::Sim, baud: u32) {
        // enable the peripherial clock for the UART
        match self.name() {
            UART0 => sim.scgc4.update(|scgc4| {
                scgc4.set_bit(10, true);
            }),
            UART1 => sim.scgc4.update(|scgc4| {
                scgc4.set_bit(11, true);
            }),
            UART2 => sim.scgc4.update(|scgc4| {
                scgc4.set_bit(12, true);
            }),
        }

        // 8 bit no parity
        self.C1.write(0);

        // set the baud rate: This has 3 components
        // BDH = Contains interrupt enable bits and the high 5 bits of the divisor
        // BDL = Contains the low 8 bits of the divisor
        // C4_BRFA = The fine adjust value
        //
        // tx baud = module clock / (16 * (divisor + BRFA / 32))
        let f_cpu = crate::mcg::F_CPU;
        let divisor = f_cpu / (baud * 16);
        let brfa = ((2 * f_cpu) / baud - divisor * 32) as u8;

        self.BDH.write((divisor >> 8) as u8 & UART_BDH_MASK);
        self.BDL.write(divisor as u8 & UART_BDL_MASK);
        self.C4.write(brfa & UART_C4_BRFA_MASK);

        // enable Tx
        self.C2.write(UART_C2_TX_ENABLE_MASK);
    }

    pub fn write_byte(&mut self, b: u8) {
        //TDRE is bit 7
        while !self.S1.read().get_bit(7) {}

        self.D.write(b);
    }
}

impl core::fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }

        while !self.S1.read().get_bit(6) {}
        Ok(())
    }
}
