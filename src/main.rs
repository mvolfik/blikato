#![no_std]
#![no_main]

use ruduino::cores::current::port::{
    B0 as dot, B4 as button, B5 as blinker, D2 as SRCLK, D4 as RLCLK, D7 as SER,
};
use ruduino::Pin;

#[no_mangle]
pub extern "C" fn main() {
    blinker::set_output();
    dot::set_output();
    button::set_input();
    SER::set_output();
    RLCLK::set_output();
    SRCLK::set_output();

    blinker::set_high();
    dot::set_low();
    set_segments(0);
    let mut on = false;
    let mut i = 0;
    let mut code = || {
        if on {
            dot::set_high();
            blinker::set_high();
        } else {
            dot::set_low();
            blinker::set_low();
        }
        set_segments(*unsafe { HEX_DIGITS.get_unchecked(i) });
        i = (i + 1) % 16;
        on = !on;
    };
    // loop_automatically(300, &mut code);
    step_by_button(&mut code);
}

fn loop_automatically(delay_ms: u64, code: &mut dyn FnMut()) {
    loop {
        blinker::set_high();
        dot::set_low();
        set_segments(0);
        while button::is_low() {}
        while button::is_high() {}
        loop {
            if button::is_high() {
                while button::is_high() {}
                break;
            }
            code();
            ruduino::delay::delay_ms(delay_ms);
        }
    }
}

fn step_by_button(code: &mut dyn FnMut()) {
    loop {
        while button::is_low() {}
        code();
        while button::is_high() {}
    }
}

const HEX_DIGITS: [u8; 16] = [
    0b1110111, // 0
    0b0010010, // 1
    0b1011101, // 2
    0b1011011, // 3
    0b0111010, // 4
    0b1101011, // 5
    0b1101111, // 6
    0b1010010, // 7
    0b1111111, // 8
    0b1111011, // 9
    0b1111110, // A
    0b0101111, // B
    0b1100101, // C
    0b0011111, // D
    0b1101101, // E
    0b1101100, // F
];

/// For bits 0bXGFEDCBA (most significant bit first), the segments are:
/// ```raw
/// +--G--+
/// |     |
/// F     E
/// |     |
/// +--D--+
/// |     |
/// C     B
/// |     |
/// +--A--+
/// ```
fn set_segments(segments: u8) {
    for i in 0..8 {
        if (segments & 128 >> i) != 0 {
            SER::set_high();
        } else {
            SER::set_low();
        }
        SRCLK::set_high();
        SRCLK::set_low();
    }
    RLCLK::set_high();
    RLCLK::set_low();
}
