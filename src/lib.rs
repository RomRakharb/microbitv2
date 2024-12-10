#![no_std]
#![no_main]

use embedded_hal::digital::{OutputPin, PinState, StatefulOutputPin};
use nrf52833_hal::{
    self as hal,
    gpio::{p0, p1, Floating, Input, Level, Output, Pin, PushPull},
    pac::Peripherals,
};

pub struct Board {
    port0: p0::Parts,
    port1: p1::Parts,
}

impl Board {
    pub fn init() -> Self {
        let p = hal::pac::Peripherals::take().unwrap();
        Self {
            port0: p0::Parts::new(p.P0),
            port1: p1::Parts::new(p.P1),
        }
    }

    pub fn led_matrix(&self) -> LedMatrix {
        LedMatrix {
            col: [
                self.port0
                    .p0_28
                    .into_push_pull_output(Level::High)
                    .degrade(),
                self.port0
                    .p0_11
                    .into_push_pull_output(Level::High)
                    .degrade(),
                self.port0
                    .p0_31
                    .into_push_pull_output(Level::High)
                    .degrade(),
                self.port1
                    .p1_05
                    .into_push_pull_output(Level::High)
                    .degrade(),
                self.port0
                    .p0_30
                    .into_push_pull_output(Level::High)
                    .degrade(),
            ],
            row: [
                self.port0.p0_21.into_push_pull_output(Level::Low).degrade(),
                self.port0.p0_22.into_push_pull_output(Level::Low).degrade(),
                self.port0.p0_15.into_push_pull_output(Level::Low).degrade(),
                self.port0.p0_24.into_push_pull_output(Level::Low).degrade(),
                self.port0.p0_19.into_push_pull_output(Level::Low).degrade(),
            ],
        }
    }

    pub fn button(self) -> Button {
        let board = self;
        Button {
            a: board.port0.p0_14.into_floating_input().degrade(),
            b: board.port0.p0_23.into_floating_input().degrade(),
        }
    }
}

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

pub struct Button {
    pub a: Pin<Input<Floating>>,
    pub b: Pin<Input<Floating>>,
}
