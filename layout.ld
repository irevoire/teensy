/*
 * First off, let's define the regions for flash and RAM.
 * The teensy 3.1/3.2 uses a MK20DX256VLH7 with 256K flash starting at 0
 *
 * The RAM is split into two halfes called SRAM_L and SRAM_U.
 * SRAM_L ends at 0x1fff ffff and SRAM_U starts with 0x2000 0000
 * for most purposes, both blocks can be used as one. However, when in
 * VLLS2 power mode, only SRAM_U is powered.
 *
 * For the 64K of RAM in the MK20DX256VLH7, this translates to a
 * start address of 0x1fff8000
 */

MEMORY
{
	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
	RAM  (rwx) : ORIGIN = 0x1FFF8000, LENGTH = 64K
}

SECTIONS
{
	.text : {
		. = 0;
		KEEP(*(.vectors))
		. = 0x400;
		KEEP(*(.flashconfig*))
		. = ALIGN(4);
		*(.text*)
    } > FLASH = 0xFF

    .rodata : {
	    *(.rodata*)
    } > FLASH

    _stack_top = ORIGIN(RAM) + LENGTH(RAM);

    /DISCARD/ : {
	    *(.ARM.*)
    }
}
