use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours_from_minutes = minutes / 60;
        let mut minutes = minutes % 60;

        if minutes < 0 {
            hours_from_minutes -= 1;
            minutes += 60;
        }

        let mut hours = (hours + hours_from_minutes) % 24;
        if hours < 0 {
            hours += 24;
        }

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let hours = self.hours + minutes / 60;
        let minutes = self.minutes + minutes % 60;

        Self::new(hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
