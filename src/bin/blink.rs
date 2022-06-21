#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{OutputPin, PinState};
use imxrt_hal::gpio::GPIO;
use teensy4_bsp as bsp;

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::with_source(
        peripherals.SYST,
        teensy4_bsp::EXT_SYSTICK_HZ,
        cortex_m::peripheral::syst::SystClkSource::External,
    );

    let peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::t40::from_pads(peripherals.iomuxc);

    let mut led = bsp::configure_led(pins.p13);
    let mut p0 = GPIO::new(pins.p0).output();

    let mut led_state = PinState::High;
    let mut p0_state = PinState::Low;

    loop {
        led.set_state(led_state).unwrap();
        p0.set_state(p0_state).unwrap();
        core::mem::swap(&mut led_state, &mut p0_state);
        delay.delay_ms(500);
    }
}
