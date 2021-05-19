use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // convert everything into minutes
        let mut m = hours*60 + minutes;
        // convert it back to hours
        // euclid division is used to round towards -inf
        // (/ operator default is to round towards 0)
        let mut h = m.div_euclid(60);
        // bring hours in 00-23 and minutes in 00-59
        // euclid remainder is used for non-negative result
        // (% operator default is a signed remainder)
        Clock {
            hours: h.rem_euclid(24),
            minutes: m.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
