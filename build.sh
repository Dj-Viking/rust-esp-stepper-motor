#! /usr/bin/bash

MONITOR=$1

# build the rust project to prepare flashing esp with the compiled rust code
cargo build

# kind of a weird way to check if we want to do monitor or not
# this may need to be adjusted for other operating systems that don't have `doas`
# because I think this requires administrative access to run the flash
if ! [ -z "$MONITOR" ]; then
	if [ "$MONITOR" = "-m" ]; then
		# TODO: check the monitor warning, im not sure if the monitor quite works...
		doas espflash flash ./target/riscv32imac-unknown-none-elf/debug/rust-esp-neopixel
		doas espflash monitor
	fi
else
	doas espflash flash ./target/riscv32imac-unknown-none-elf/debug/rust-esp-neopixel
fi
