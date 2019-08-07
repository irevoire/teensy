//! Here is the most important file of this crate.
//! When you add this crate as a dependency it will move the bootloader to the good section. Enable
//! **all** the port, the clock at 72MHz and disable the watchdog and **then** call your `main`.
//! Right now if you don't want something you'll need to either disable it in your `main` or edit
//! this crate.

use crate::*;

/// The first function to be executed by the teensy
/// Enable all the clocks:
/// - Core: 72MHz
/// - Peripherals: 36MHz
/// - Flash: 24MHz
/// Enable all the ports clock gate. This may consume more power than what you need if you don't
/// use all the ports.
/// Disable the watchdog.
#[no_mangle]
extern "C" fn __boot() {
    let (wdog, sim, mcg, osc) = unsafe {
        (
            watchdog::Watchdog::new(),
            sim::Sim::new(),
            mcg::Mcg::new(),
            osc::Osc::new(),
        )
    };

    wdog.disable();

    // Enable the crystal oscillator with 10pf of capacitance
    osc.enable(10);
    // Turn on all the port clock gate
    sim.enable_clock(sim::Clock::PortA);
    sim.enable_clock(sim::Clock::PortB);
    sim.enable_clock(sim::Clock::PortC);
    sim.enable_clock(sim::Clock::PortD);
    sim.enable_clock(sim::Clock::PortE);

    // Set our clocks:
    // core: 72Mhz
    // peripheral: 36MHz
    // flash: 24MHz
    sim.set_dividers(1, 2, 3);
    // We would also set the USB divider here if we wanted to use it.

    // Now we can start setting up the MCG for our needs.
    if let mcg::Clock::Fei(mut fei) = mcg.clock() {
        // Our 16MHz xtal is "very fast", and needs to be divided
        // by 512 to be in the acceptable FLL range.
        fei.enable_xtal(mcg::OscRange::VeryHigh);
        let fbe = fei.use_external(512);

        // PLL is 27/6 * xtal == 72MHz
        let pbe = fbe.enable_pll(27, 6);
        pbe.use_pll();
    } else {
        panic!("Somehow the clock wasn't in FEI mode");
    }

    unsafe {
        main();
    }

    core::panic!("Came out of main");
}

/// This is the Interrupt Descriptor Table
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static _INTERRUPTS: [unsafe extern "C" fn(); 111] = [
    __boot, // TODO: Move this to a different vector?
    interrupts::isr_non_maskable,
    interrupts::isr_hard_fault,
    interrupts::isr_memmanage_fault,
    interrupts::isr_bus_fault,
    interrupts::isr_usage_fault,
    interrupts::isr_ignore_no_args, // Reserved 7
    interrupts::isr_ignore_no_args, // Reserved 8
    interrupts::isr_ignore_no_args, // Reserved 9
    interrupts::isr_ignore_no_args, // Reserved 10
    interrupts::isr_svcall,
    interrupts::isr_debug_monitor,
    interrupts::isr_ignore_no_args, // Reserved 13
    interrupts::isr_pendablesrvreq,
    interrupts::isr_systick,
    interrupts::isr_dma_ch0_complete,
    interrupts::isr_dma_ch1_complete,
    interrupts::isr_dma_ch2_complete,
    interrupts::isr_dma_ch3_complete,
    interrupts::isr_dma_ch4_complete,
    interrupts::isr_dma_ch5_complete,
    interrupts::isr_dma_ch6_complete,
    interrupts::isr_dma_ch7_complete,
    interrupts::isr_dma_ch8_complete,
    interrupts::isr_dma_ch9_complete,
    interrupts::isr_dma_ch10_complete,
    interrupts::isr_dma_ch11_complete,
    interrupts::isr_dma_ch12_complete,
    interrupts::isr_dma_ch13_complete,
    interrupts::isr_dma_ch14_complete,
    interrupts::isr_dma_ch15_complete,
    interrupts::isr_dma_error,
    interrupts::isr_ignore_no_args, // Unused ? INT_MCM ?
    interrupts::isr_flash_cmd_complete,
    interrupts::isr_flash_read_collision,
    interrupts::isr_low_voltage_warning,
    interrupts::isr_low_voltage_wakeup,
    interrupts::isr_wdog_or_emw,
    interrupts::isr_ignore_no_args, // Reserved 39
    interrupts::isr_i2c0,
    interrupts::isr_i2c1,
    interrupts::isr_spi0,
    interrupts::isr_spi1,
    interrupts::isr_ignore_no_args, // Teensy does not have SPI2
    interrupts::isr_can0_or_msg_buf,
    interrupts::isr_can0_bus_off,
    interrupts::isr_can0_error,
    interrupts::isr_can0_transmit_warn,
    interrupts::isr_can0_receive_warn,
    interrupts::isr_can0_wakeup,
    interrupts::isr_i2s0_transmit,
    interrupts::isr_i2s0_receive,
    interrupts::isr_ignore_no_args, // Teensy does not have CAN1
    interrupts::isr_ignore_no_args, // Teensy does not have CAN1
    interrupts::isr_ignore_no_args, // Teensy does not have CAN1
    interrupts::isr_ignore_no_args, // Teensy does not have CAN1
    interrupts::isr_ignore_no_args, // Teensy does not have CAN1
    interrupts::isr_ignore_no_args, // Teensy does not have CAN1
    interrupts::isr_ignore_no_args, // Reserved 59
    interrupts::isr_uart0_lon,
    interrupts::isr_uart0_status,
    interrupts::isr_uart0_error,
    interrupts::isr_uart1_status,
    interrupts::isr_uart1_error,
    interrupts::isr_uart2_status,
    interrupts::isr_uart2_error,
    interrupts::isr_ignore_no_args, // Teensy does not have UART3
    interrupts::isr_ignore_no_args, // Teensy does not have UART3
    interrupts::isr_ignore_no_args, // Teensy does not have UART4
    interrupts::isr_ignore_no_args, // Teensy does not have UART4
    interrupts::isr_ignore_no_args, // Teensy does not have UART5
    interrupts::isr_ignore_no_args, // Teensy does not have UART5
    interrupts::isr_adc0,
    interrupts::isr_adc1,
    interrupts::isr_cmp0,
    interrupts::isr_cmp1,
    interrupts::isr_cmp2,
    interrupts::isr_ftm0,
    interrupts::isr_ftm1,
    interrupts::isr_ftm2,
    interrupts::isr_cmt,
    interrupts::isr_rtc_alarm,
    interrupts::isr_rtc_seconds,
    interrupts::isr_pit_ch0,
    interrupts::isr_pit_ch1,
    interrupts::isr_pit_ch2,
    interrupts::isr_pit_ch3,
    interrupts::isr_pdb,
    interrupts::isr_usb_otg,
    interrupts::isr_usb_charger,
    interrupts::isr_ignore_no_args, // Reserved 91
    interrupts::isr_ignore_no_args, // Reserved 92
    interrupts::isr_ignore_no_args, // Reserved 93
    interrupts::isr_ignore_no_args, // Reserved 94
    interrupts::isr_ignore_no_args, // Nothing according to manual, I2S0 according to headers
    interrupts::isr_ignore_no_args, // Nothing according to manual, SDHC according to headers
    interrupts::isr_dac0,
    interrupts::isr_ignore_no_args, // Teensy does not have DAC1
    interrupts::isr_tsi,
    interrupts::isr_mcg,
    interrupts::isr_lpt,
    interrupts::isr_ignore_no_args, // Reserved 102
    interrupts::isr_port_a,
    interrupts::isr_port_b,
    interrupts::isr_port_c,
    interrupts::isr_port_d,
    interrupts::isr_port_e,
    interrupts::isr_ignore_no_args, // Reserved 108
    interrupts::isr_ignore_no_args, // Reserved 109
    interrupts::isr_software,
    interrupts::isr_ignore_no_args, // Reserved 111
];

/// Flash configuration
/// Controls how the flash can be read or written.
///  The Teensy bootloader makes assumptions about these values, so
/// we will use the same set of bytes as the Teensy Arduino tooling.
/// Specifically, we disable all flash security through the FSEC
/// field, and tell the processor to boot into high-power mode with FOPT.
#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF,
    //                      FSEC, FOPT
];
