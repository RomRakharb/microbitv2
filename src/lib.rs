#![no_std]
#![no_main]

use core::panic;

use embedded_hal::digital::{OutputPin, PinState, StatefulOutputPin};
use nrf52833_hal::{
    self as hal,
    gpio::{p0, p1, Level, Output, Pin, PushPull},
};
use rtt_target::{rprint, rprintln};
enum Action {
    ShiftRight,
    ShiftLeft,
    SetRow(bool, bool, bool, bool, bool),
}

pub struct LedMatrix {
    col: [Pin<Output<PushPull>>; 5],
    row: [Pin<Output<PushPull>>; 5],
}

impl LedMatrix {
    pub fn init() -> Self {
        let p = hal::pac::Peripherals::take().unwrap();
        let port0 = p0::Parts::new(p.P0);
        let port1 = p1::Parts::new(p.P1);

        Self {
            col: [
                port0.p0_28.into_push_pull_output(Level::High).degrade(),
                port0.p0_11.into_push_pull_output(Level::High).degrade(),
                port0.p0_31.into_push_pull_output(Level::High).degrade(),
                port1.p1_05.into_push_pull_output(Level::High).degrade(),
                port0.p0_30.into_push_pull_output(Level::High).degrade(),
            ],
            row: [
                port0.p0_21.into_push_pull_output(Level::Low).degrade(),
                port0.p0_22.into_push_pull_output(Level::Low).degrade(),
                port0.p0_15.into_push_pull_output(Level::Low).degrade(),
                port0.p0_24.into_push_pull_output(Level::Low).degrade(),
                port0.p0_19.into_push_pull_output(Level::Low).degrade(),
            ],
        }
    }

    pub fn set_state(&mut self, col: [bool; 5], row: [bool; 5]) {
        for i in 0..5 {
            let _ = self.col[i].set_state(PinState::from(col[i]));
            let _ = self.row[i].set_state(PinState::from(row[i]));
        }
    }

    pub fn get_state(&mut self) -> ([bool; 5], [bool; 5]) {
        let mut col: [bool; 5] = [true; 5];
        let mut row: [bool; 5] = [false; 5];
        for i in 0..5 {
            col[i] = self.col[i].is_set_high().unwrap();
            row[i] = self.row[i].is_set_high().unwrap();
        }
        (col, row)
    }

    pub fn shift_right(&mut self) {
        let mut exceed_col = true;
        for i in (0..5).rev() {
            if self.col[i].is_set_low().unwrap() {
                let _ = self.col[i].set_high();
                if i == 4 {
                    exceed_col = false;
                } else {
                    let _ = self.col[i + 1].set_low();
                }
            }
        }
        let _ = self.col[0].set_state(PinState::from(exceed_col));
    }

    pub fn shift_left(&mut self) {
        let mut exceed_col = true;
        for i in 0..5 {
            if self.col[i].is_set_low().unwrap() {
                let _ = self.col[i].set_high();
                if i == 0 {
                    exceed_col = false;
                } else {
                    let _ = self.col[i - 1].set_low();
                }
            }
        }
        let _ = self.col[4].set_state(PinState::from(exceed_col));
    }
}
