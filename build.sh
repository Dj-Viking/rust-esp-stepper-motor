#! /usr/bin/bash
MONITOR=$1

cargo build

doas espflash flash ./target/riscv32imac-unknown-none-elf/debug/rust-esp-neopixel

if ! [ -z "$MONITOR" ]; then
	if [ "$MONITOR" = "-m" ]; then
		doas espflash monitor
	fi
fi
