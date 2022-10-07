# blikato

"Bliká to". My first minimal working project where I write Rust to run on an Arduino.

## Hardware setup + wire connections

Undocumented

## Build + upload to Arduino

`cargo b --release && avrdude -patmega328p -carduino -P/dev/ttyUSB0 -D -Uflash:w:target/avr-unknown-gnu-atmega328p/release/blink.elf:e`
