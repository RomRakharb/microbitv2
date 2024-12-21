#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin, PinState, StatefulOutputPin};
use nrf52833_hal::{
    self as hal,
    gpio::{p0, p1, Floating, Input, Level, Output, Pin, PushPull},
    pac::Peripherals,
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

// mod time;
mod day1;
use microbitv2::{Action, Board, Buttons, LedMatrix, Transition};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::init();
    let mut led = board.led_matrix;
    let mut buttons = board.buttons;

    led.set_state(
        [false, true, true, true, true],
        [true, false, false, false, false],
    );

    loop {
        led.process(Action::Render("12", Some(Transition::Right)));
        led.process(Action::Render("34", Some(Transition::Down)));
        led.process(Action::Render("56", Some(Transition::Left)));
        led.process(Action::Render("78", Some(Transition::Up)));
        led.process(Action::Render("90", None));
        // for _ in 0..400_000 {
        //     nop();
        // }
        // led.shift_right();
        // led.render();
        // if buttons.a.is_low().unwrap() {}
        // if buttons.b.is_low().unwrap() {}
    }
}
