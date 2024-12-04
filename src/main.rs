#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use nrf52833_hal::{
    self as hal,
    gpio::{p0, p1, Level, Output, PushPull},
};
use panic_halt as _;
use rtt_target::rtt_init_print;

struct State<T> {
    led: T,
    state: bool,
}

impl<T> State<T> {
    fn from(led: T, state: bool) -> Self {
        State { led, state }
    }
}

struct RowOne<'a>(
    State<&'a mut p0::P0_28<Output<PushPull>>>,
    State<&'a mut p0::P0_11<Output<PushPull>>>,
    State<&'a mut p0::P0_31<Output<PushPull>>>,
    State<&'a mut p1::P1_05<Output<PushPull>>>,
    State<&'a mut p0::P0_30<Output<PushPull>>>,
);

impl RowOne<'_> {
    fn process(&mut self, action: Action) {
        match action {
            Action::ShiftRight => {
                if self.0.state == true {
                    self.0.state = false;
                    self.1.state = true
                } else if self.1.state == true {
                    self.1.state = false;
                    self.2.state = true
                } else if self.2.state == true {
                    self.2.state = false;
                    self.3.state = true
                } else if self.3.state == true {
                    self.3.state = false;
                    self.4.state = true
                } else if self.4.state == true {
                    self.4.state = false;
                    self.0.state = true
                }
            }
            Action::ShiftLeft => {
                if self.4.state == true {
                    self.4.state = false;
                    self.3.state = true
                } else if self.3.state == true {
                    self.3.state = false;
                    self.2.state = true
                } else if self.2.state == true {
                    self.2.state = false;
                    self.1.state = true
                } else if self.1.state == true {
                    self.1.state = false;
                    self.0.state = true
                } else if self.0.state == true {
                    self.0.state = false;
                    self.4.state = true
                }
            }
        }
    }

    fn output(&mut self) {
        let _ = self.0.led.set_state(PinState::from(!self.0.state));
        let _ = self.1.led.set_state(PinState::from(!self.1.state));
        let _ = self.2.led.set_state(PinState::from(!self.2.state));
        let _ = self.3.led.set_state(PinState::from(!self.3.state));
        let _ = self.4.led.set_state(PinState::from(!self.4.state));
    }
}

#[allow(dead_code)]
enum Action {
    ShiftRight,
    ShiftLeft,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.P0);
    let port1 = p1::Parts::new(p.P1);

    let mut col1 = port0.p0_28.into_push_pull_output(Level::High);
    let mut col2 = port0.p0_11.into_push_pull_output(Level::High);
    let mut col3 = port0.p0_31.into_push_pull_output(Level::High);
    let mut col4 = port1.p1_05.into_push_pull_output(Level::High);
    let mut col5 = port0.p0_30.into_push_pull_output(Level::High);
    let _row1 = port0.p0_21.into_push_pull_output(Level::High);

    // let mut button_a = port0.p0_14.into_pullup_input();

    let mut leds: RowOne = RowOne(
        State::from(&mut col1, true),
        State::from(&mut col2, false),
        State::from(&mut col3, false),
        State::from(&mut col4, false),
        State::from(&mut col5, false),
    );

    loop {
        leds.output();
        leds.process(Action::ShiftRight);
        for _ in 0..200_000 {
            nop();
        }
    }
}
