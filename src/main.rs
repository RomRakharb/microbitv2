#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

// mod time;
use microbitv2::Board;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // let board = Board::init();
    let mut led = Board::init().led_matrix();
    // let mut buttons = Board::init().button();

    led.set_state(
        [false, true, true, true, true],
        [true, false, false, false, false],
    );

    loop {
        for _ in 0..400_000 {
            nop();
        }
        led.shift_right();
        // if buttons.a.is_low().unwrap() {}
        // if buttons.b.is_low().unwrap() {}
    }
}
