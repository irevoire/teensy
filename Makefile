OUTDIR=target/thumbv7em-none-eabi/release/examples

# You can build an example by doing `ex_blink` for example
ex_%:
	cargo build --release --example $*

# This build the hexadecimal version of an example
%.hex: ex_%
	arm-none-eabi-objcopy -O ihex ${OUTDIR}/$* ${OUTDIR}/$@

# This is the function used to flash your teensy3.2 with an example.
# For example to flash the blink example you can run `flash_blink`.
# All the accepted examples are in the examples directory
flash_%: %.hex
	teensy_loader_cli -w -mmcu=mk20dx256 ${OUTDIR}/$< -v
