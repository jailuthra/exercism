// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Debug)]
pub struct Duration {
    earth_years: f64,    
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { earth_years: (s as f64)/(31557600.00 as f64) }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
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

macro_rules! orbital_period {
    ( $( $struct:ident => $years:expr ),+ ) => {
        $(
            impl $struct {
                const ORBITAL_PERIOD: Duration = Duration {earth_years: $years};
            }
        )+
    };
}

orbital_period!(Earth => 1.0, Mercury =>  0.2408467, Venus => 0.61519726, Mars => 1.8808158, Jupiter => 11.862615,
    Saturn => 29.447498, Uranus => 84.016846, Neptune => 164.79132);

macro_rules! planet {
    ( $( $struct:ident ),+ ) => {
        $(
            impl Planet for $struct {
                fn years_during(d: &Duration) -> f64 {
                    d.earth_years / Self::ORBITAL_PERIOD.earth_years
                }
            }
        )+
    };
}

planet!(Earth, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune);

