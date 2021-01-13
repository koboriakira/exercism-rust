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
    const RATIO: f64 = 1.0;
    fn years_during(d: &Duration) -> f64 {
        round_second_decimal_place(d.earth_age() / Self::RATIO)
    }
}

macro_rules! create_planet {
    ($planet_name: ident, $ratio:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            const RATIO: f64 = $ratio;
        }
    };
}
create_planet!(Mercury, 0.2408467);
create_planet!(Venus, 0.61519726);
create_planet!(Earth, 1.0);
create_planet!(Mars, 1.8808158);
create_planet!(Jupiter, 11.862615);
create_planet!(Saturn, 29.447498);
create_planet!(Uranus, 84.016846);
create_planet!(Neptune, 164.79132);
