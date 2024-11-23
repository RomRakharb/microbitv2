#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let mut x = 0;
    loop {
        x += 1;
        for _ in 0..x {
            nop();
        }
    }
    // rtt_init_print!();
    // rprintln!("Hello, world!");
    // loop {
    //     rprintln!("Echo..");
    //     for _ in 0..100_000 {
    //         nop();
    //     }
    // }
}
