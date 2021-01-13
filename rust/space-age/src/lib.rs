// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    value: u64,
    _secret: (),
}

impl Duration {
    fn earth_age(&self) -> f64 {
        const EARTH_DAYS: u64 = 31_557_600;
        self.value as f64 / EARTH_DAYS as f64
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            value: s,
            _secret: (),
        }
    }
}

fn round_second_decimal_place(f: f64) -> f64 {
    (f * 100 as f64).round() / 100 as f64
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age())
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 0.240_846_7)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 0.615_197_26)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age())
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 1.8808158)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 11.862615)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 29.447498)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 84.016846)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / 164.79132)
    }
}
