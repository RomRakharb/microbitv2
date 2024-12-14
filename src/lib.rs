#![no_std]
#![no_main]

use cortex_m::asm::nop;
use embedded_hal::digital::{OutputPin, PinState, StatefulOutputPin};
use nrf52833_hal::{
    self as hal,
    gpio::{p0, p1, Floating, Input, Level, Output, Pin, PushPull},
};

mod character;
use character::{number::*, Character, BLANK};

pub struct Board {
    pub led_matrix: LedMatrix,
    pub buttons: Buttons,
}

impl Board {
    pub fn init() -> Self {
        let p = hal::pac::Peripherals::take().unwrap();
        let port0 = p0::Parts::new(p.P0);
        let port1 = p1::Parts::new(p.P1);

        Self {
            led_matrix: LedMatrix {
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
            },

            buttons: Buttons {
                a: port0.p0_14.into_floating_input().degrade(),
                b: port0.p0_23.into_floating_input().degrade(),
            },
        }
    }
}

pub enum Action<'a> {
    ShiftRight,
    ShiftLeft,
    Render(&'a str),
}

pub struct LedMatrix {
    pub col: [Pin<Output<PushPull>>; 5],
    pub row: [Pin<Output<PushPull>>; 5],
}

impl LedMatrix {
    pub fn get_state(&mut self) -> ([bool; 5], [bool; 5]) {
        let mut col: [bool; 5] = [true; 5];
        let mut row: [bool; 5] = [false; 5];
        for i in 0..5 {
            col[i] = self.col[i].is_set_high().unwrap();
            row[i] = self.row[i].is_set_high().unwrap();
        }
        (col, row)
    }

    pub fn set_state(&mut self, col: [bool; 5], row: [bool; 5]) {
        for i in 0..5 {
            let _ = self.col[i].set_state(PinState::from(col[i]));
            let _ = self.row[i].set_state(PinState::from(row[i]));
        }
    }

    pub fn shift_right(&mut self) {
        let (mut col, row) = self.get_state();
        col.rotate_right(1);
        self.set_state(col, row);
    }

    pub fn shift_left(&mut self) {
        let (mut col, row) = self.get_state();
        col.rotate_left(1);
        self.set_state(col, row);
    }

    pub fn render(&mut self, character: &str) {
        let mut each_char = character.chars();
        while let Some(char) = each_char.next() {
            let render_char = match char {
                '0' => ZERO,
                '1' => ONE,
                '2' => TWO,
                '3' => THREE,
                '4' => FOUR,
                '5' => FIVE,
                '6' => SIX,
                '7' => SEVEN,
                '8' => EIGHT,
                '9' => NINE,
                _ => BLANK,
            };

            for _ in 0..100 {
                for (index, col) in render_char.0.iter().enumerate() {
                    let mut row = [false; 5];
                    row[index] = true;
                    self.set_state(*col, row);
                    for _ in 0..1_000 {
                        nop();
                    }
                }
            }
        }
    }

    pub fn process(&mut self, action: Action) {
        match action {
            Action::Render(character) => self.render(character),
            _ => {}
        }
    }
}

pub struct Buttons {
    pub a: Pin<Input<Floating>>,
    pub b: Pin<Input<Floating>>,
}

impl Buttons {
    pub fn new(a: Pin<Input<Floating>>, b: Pin<Input<Floating>>) -> Self {
        Self { a, b }
    }
}
