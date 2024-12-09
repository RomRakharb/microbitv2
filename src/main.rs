#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin, PinState, StatefulOutputPin};
use nrf52833_hal::{
    self as hal,
    gpio::{p0, p1, Level, Output, PushPull},
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

mod time;

use microbitv2::LedMatrix;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut led = LedMatrix::init();

    led.set_state(
        [false, false, true, true, true],
        [true, true, false, false, false],
    );
    loop {
        for _ in 0..400_000 {
            nop();
        }
        led.shift_left();
    }
}
