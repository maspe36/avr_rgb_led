use arduino_mega2560::prelude::*;

pub struct RgbLed<R, G, B> {
    _delay: arduino_mega2560::Delay,
    _r_pwm: R,
    _g_pwm: G,
    _b_pwm: B,
    r: u8,
    g: u8,
    b: u8
}

impl<R, G, B> RgbLed<R, G, B> {
    pub fn new(r_pwm: R, g_pwm: G, b_pwm: B) -> Self {
        r_pwm.set_duty(128);
        r_pwm.enable();

        g_pwm.set_duty(128);
        g_pwm.enable();

        b_pwm.set_duty(128);
        b_pwm.enable();

        RgbLed {
            _delay: arduino_mega2560::Delay::new(),
            _r_pwm: r_pwm,
            _g_pwm: g_pwm,
            _b_pwm: b_pwm,
            r: 0,
            b: 0,
            g: 0
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;

        delay.delay_ms(10_u16);
    }
}
