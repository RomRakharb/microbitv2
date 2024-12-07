use nrf52833_hal::{pac::RTC0, Rtc};

pub struct Ticker {
    rtc: Rtc<RTC0>,
}

impl Ticker {
    pub fn new(rtc0: RTC0) -> Self {
        let rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();
        Self { rtc }
    }

    pub fn now(&self) -> u32 {
        self.rtc.get_counter()
    }
}
