pub unsafe extern "C" fn isr_panic(_level: u8) -> ! {
    loop {}
}

pub unsafe extern "C" fn isr_ignore(_level: u8) -> () {}
pub unsafe extern "C" fn isr_ignore_no_args() -> () {}

pub unsafe extern "C" fn isr_non_maskable() {
    isr_panic(2)
}
pub unsafe extern "C" fn isr_hard_fault() {
    isr_panic(3)
}
pub unsafe extern "C" fn isr_memmanage_fault() {
    isr_panic(4)
}
pub unsafe extern "C" fn isr_bus_fault() {
    isr_panic(5)
}
pub unsafe extern "C" fn isr_usage_fault() {
    isr_panic(6)
}

pub unsafe extern "C" fn isr_svcall() {
    isr_ignore(11)
}
pub unsafe extern "C" fn isr_debug_monitor() {
    isr_ignore(12)
}

pub unsafe extern "C" fn isr_pendablesrvreq() {
    isr_ignore(14)
}
pub unsafe extern "C" fn isr_systick() {
    isr_ignore(15)
}
pub unsafe extern "C" fn isr_dma_ch0_complete() {
    isr_ignore(16)
}
pub unsafe extern "C" fn isr_dma_ch1_complete() {
    isr_ignore(17)
}
pub unsafe extern "C" fn isr_dma_ch2_complete() {
    isr_ignore(18)
}
pub unsafe extern "C" fn isr_dma_ch3_complete() {
    isr_ignore(19)
}
pub unsafe extern "C" fn isr_dma_ch4_complete() {
    isr_ignore(20)
}
pub unsafe extern "C" fn isr_dma_ch5_complete() {
    isr_ignore(21)
}
pub unsafe extern "C" fn isr_dma_ch6_complete() {
    isr_ignore(22)
}
pub unsafe extern "C" fn isr_dma_ch7_complete() {
    isr_ignore(23)
}
pub unsafe extern "C" fn isr_dma_ch8_complete() {
    isr_ignore(24)
}
pub unsafe extern "C" fn isr_dma_ch9_complete() {
    isr_ignore(25)
}
pub unsafe extern "C" fn isr_dma_ch10_complete() {
    isr_ignore(26)
}
pub unsafe extern "C" fn isr_dma_ch11_complete() {
    isr_ignore(27)
}
pub unsafe extern "C" fn isr_dma_ch12_complete() {
    isr_ignore(28)
}
pub unsafe extern "C" fn isr_dma_ch13_complete() {
    isr_ignore(29)
}
pub unsafe extern "C" fn isr_dma_ch14_complete() {
    isr_ignore(30)
}
pub unsafe extern "C" fn isr_dma_ch15_complete() {
    isr_ignore(31)
}
pub unsafe extern "C" fn isr_dma_error() {
    isr_ignore(32)
}

pub unsafe extern "C" fn isr_flash_cmd_complete() {
    isr_ignore(34)
}
pub unsafe extern "C" fn isr_flash_read_collision() {
    isr_ignore(35)
}
pub unsafe extern "C" fn isr_low_voltage_warning() {
    isr_ignore(36)
}
pub unsafe extern "C" fn isr_low_voltage_wakeup() {
    isr_ignore(37)
}
pub unsafe extern "C" fn isr_wdog_or_emw() {
    isr_ignore(38)
}

pub unsafe extern "C" fn isr_i2c0() {
    isr_ignore(40)
}
pub unsafe extern "C" fn isr_i2c1() {
    isr_ignore(41)
}
pub unsafe extern "C" fn isr_spi0() {
    isr_ignore(42)
}
pub unsafe extern "C" fn isr_spi1() {
    isr_ignore(43)
}

pub unsafe extern "C" fn isr_can0_or_msg_buf() {
    isr_ignore(45)
}
pub unsafe extern "C" fn isr_can0_bus_off() {
    isr_ignore(46)
}
pub unsafe extern "C" fn isr_can0_error() {
    isr_ignore(47)
}
pub unsafe extern "C" fn isr_can0_transmit_warn() {
    isr_ignore(48)
}
pub unsafe extern "C" fn isr_can0_receive_warn() {
    isr_ignore(49)
}
pub unsafe extern "C" fn isr_can0_wakeup() {
    isr_ignore(50)
}
pub unsafe extern "C" fn isr_i2s0_transmit() {
    isr_ignore(51)
}
pub unsafe extern "C" fn isr_i2s0_receive() {
    isr_ignore(52)
}

pub unsafe extern "C" fn isr_uart0_lon() {
    isr_ignore(60)
}
pub unsafe extern "C" fn isr_uart0_status() {
    isr_ignore(61)
}
pub unsafe extern "C" fn isr_uart0_error() {
    isr_ignore(62)
}
pub unsafe extern "C" fn isr_uart1_status() {
    isr_ignore(63)
}
pub unsafe extern "C" fn isr_uart1_error() {
    isr_ignore(64)
}
pub unsafe extern "C" fn isr_uart2_status() {
    isr_ignore(65)
}
pub unsafe extern "C" fn isr_uart2_error() {
    isr_ignore(66)
}

pub unsafe extern "C" fn isr_adc0() {
    isr_ignore(73)
}
pub unsafe extern "C" fn isr_adc1() {
    isr_ignore(74)
}
pub unsafe extern "C" fn isr_cmp0() {
    isr_ignore(75)
}
pub unsafe extern "C" fn isr_cmp1() {
    isr_ignore(76)
}
pub unsafe extern "C" fn isr_cmp2() {
    isr_ignore(77)
}
pub unsafe extern "C" fn isr_ftm0() {
    isr_ignore(78)
}
pub unsafe extern "C" fn isr_ftm1() {
    isr_ignore(79)
}
pub unsafe extern "C" fn isr_ftm2() {
    isr_ignore(80)
}
pub unsafe extern "C" fn isr_cmt() {
    isr_ignore(81)
}
pub unsafe extern "C" fn isr_rtc_alarm() {
    isr_ignore(82)
}
pub unsafe extern "C" fn isr_rtc_seconds() {
    isr_ignore(83)
}
pub unsafe extern "C" fn isr_pit_ch0() {
    isr_ignore(84)
}
pub unsafe extern "C" fn isr_pit_ch1() {
    isr_ignore(85)
}
pub unsafe extern "C" fn isr_pit_ch2() {
    isr_ignore(86)
}
pub unsafe extern "C" fn isr_pit_ch3() {
    isr_ignore(87)
}
pub unsafe extern "C" fn isr_pdb() {
    isr_ignore(88)
}
pub unsafe extern "C" fn isr_usb_otg() {
    isr_ignore(89)
}
pub unsafe extern "C" fn isr_usb_charger() {
    isr_ignore(90)
}

pub unsafe extern "C" fn isr_dac0() {
    isr_ignore(97)
}

pub unsafe extern "C" fn isr_tsi() {
    isr_ignore(99)
}
pub unsafe extern "C" fn isr_mcg() {
    isr_ignore(100)
}
pub unsafe extern "C" fn isr_lpt() {
    isr_ignore(101)
}

pub unsafe extern "C" fn isr_port_a() {
    isr_ignore(103)
}
pub unsafe extern "C" fn isr_port_b() {
    isr_ignore(104)
}
pub unsafe extern "C" fn isr_port_c() {
    isr_ignore(105)
}
pub unsafe extern "C" fn isr_port_d() {
    isr_ignore(106)
}
pub unsafe extern "C" fn isr_port_e() {
    isr_ignore(107)
}

pub unsafe extern "C" fn isr_software() {
    isr_ignore(110)
}
