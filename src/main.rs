#![no_std]
#![no_main]

mod led;

extern crate panic_halt;

use arduino_mega2560::pwm;
use arduino_mega2560::prelude::*;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();
    let mut pins = arduino_mega2560::Pins::new(
        dp.PORTA,
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
        dp.PORTF,
        dp.PORTG,
        dp.PORTH,
        dp.PORTJ,
        dp.PORTK,
        dp.PORTL,
    );

    let mut timer0 = pwm::Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);
    let mut timer1 = pwm::Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale64);

    let mut r_out = pins.d13.into_output(&mut pins.ddr);
    let mut g_out = pins.d12.into_output(&mut pins.ddr);
    let mut b_out = pins.d11.into_output(&mut pins.ddr);

    r_out.set_high().void_unwrap();
    g_out.set_low().void_unwrap();
    b_out.set_low().void_unwrap();

    let mut r_pwm = r_out.into_pwm(&mut timer0);
    let mut g_pwm = g_out.into_pwm(&mut timer1);
    let mut b_pwm = b_out.into_pwm(&mut timer1);

    r_pwm.set_duty(128);
    r_pwm.enable();

    g_pwm.set_duty(128);
    g_pwm.enable();

    b_pwm.set_duty(128);
    b_pwm.enable();

    loop {
        for i in 0..=255_u8 {
            r_pwm.set_duty(i);
            delay.delay_ms(10_u16);
        }

        for i in 0..=255_u8 {
            g_pwm.set_duty(i);
            delay.delay_ms(10_u16);
        }

        for i in 0..=255_u8 {
            b_pwm.set_duty(i);
            delay.delay_ms(10_u16);
        }
    }
}
