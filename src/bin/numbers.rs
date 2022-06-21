#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{OutputPin, PinState};
use imxrt_hal::{
    gpio::{Output, GPIO},
    iomuxc::prelude::gpio::Pin,
};
use teensy4_bsp as bsp;

fn to_segments(number: usize) -> [bool; 7] {
    [
        [0, 2, 3, 5, 6, 7, 8, 9].iter().any(|&x| x == number),
        [0, 1, 2, 3, 4, 7, 8, 9].iter().any(|&x| x == number),
        [0, 1, 3, 4, 5, 6, 7, 8, 9].iter().any(|&x| x == number),
        [0, 2, 3, 5, 6, 8, 9].iter().any(|&x| x == number),
        [0, 2, 6, 8].iter().any(|&x| x == number),
        [0, 4, 5, 6, 7, 8, 9].iter().any(|&x| x == number),
        [2, 3, 4, 5, 6, 8, 9].iter().any(|&x| x == number),
    ]
}

fn to_output<P: Pin>(pin: P) -> GPIO<P, Output> {
    GPIO::new(pin).output()
}

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

    let mut digit_0 = to_output(pins.p0);
    let mut digit_1 = to_output(pins.p1);
    let mut digit_2 = to_output(pins.p2);
    let mut digit_3 = to_output(pins.p3);

    let mut seg_a = to_output(pins.p5);
    let mut seg_b = to_output(pins.p6);
    let mut seg_c = to_output(pins.p7);
    let mut seg_d = to_output(pins.p8);
    let mut seg_e = to_output(pins.p9);
    let mut seg_f = to_output(pins.p10);
    let mut seg_g = to_output(pins.p11);

    let to_pinstate = |state: bool| if state { PinState::High } else { PinState::Low };

    loop {
        for i in 0..=9999 {
            for j in 0..100 {
                let digit = match j % 4 {
                    0 => i / 1000,
                    1 => i / 100 % 10,
                    2 => i / 10 % 10,
                    3 => i % 10,
                    _ => unreachable!(),
                };
                let [a, b, c, d, e, f, g] = to_segments(digit);
                let (d_0, d_1, d_2, d_3) = (j % 4 == 0, j % 4 == 1, j % 4 == 2, j % 4 == 3);
                digit_0.set_state(to_pinstate(d_0)).unwrap();
                digit_1.set_state(to_pinstate(d_1)).unwrap();
                digit_2.set_state(to_pinstate(d_2)).unwrap();
                digit_3.set_state(to_pinstate(d_3)).unwrap();
                seg_a.set_state(to_pinstate(!a)).unwrap();
                seg_b.set_state(to_pinstate(!b)).unwrap();
                seg_c.set_state(to_pinstate(!c)).unwrap();
                seg_d.set_state(to_pinstate(!d)).unwrap();
                seg_e.set_state(to_pinstate(!e)).unwrap();
                seg_f.set_state(to_pinstate(!f)).unwrap();
                seg_g.set_state(to_pinstate(!g)).unwrap();
                delay.delay_ms(1);
            }
        }
    }
}
