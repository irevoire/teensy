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
	/*think about splitting up the RAM into SRAM_U and SRAM_L*/
	RAM  (rwx) : ORIGIN = 0x1FFF8000, LENGTH = 64K
}

EXTERN(_INTERRUPTS);

SECTIONS
{
    PROVIDE(_stack_top = ORIGIN(RAM) + LENGTH(RAM));

	.text ORIGIN(FLASH) : {
        /* Initial stack pointer */
        LONG(_stack_top);

        /* TODO: add other items from vector_table before interrupts */
        /* Interrupts */
        KEEP(*(.vector_table.interrupts));
		. = 0x400;
		KEEP(*(.flashconfig*))
		. = ALIGN(4);
		*(.text*)
    } > FLASH

    .rodata : ALIGN(4){
	    *(.rodata .rodata.*);
	    . = ALIGN(4);
    } > FLASH

    /* uninitialized static vars */
    .bss : ALIGN(4)
    {
        _sbss = .;
        *(.bss.*);
        _ebss = ALIGN(4);
    } > RAM

    /* initialized static vars */
    .data : ALIGN(4)
    {
        _sdata = .;
        *(.data.*);
        _edata = ALIGN(4);
    } > RAM AT > FLASH
    _sidata = LOADADDR(.data);

    /DISCARD/ : {
	    *(.ARM.*)
    }
}
