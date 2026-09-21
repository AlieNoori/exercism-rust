// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

pub use planet_derive::Planet;

#[derive(Debug)]
pub struct Duration {
    secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { secs: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

#[derive(Planet)]
pub struct Mercury;

#[derive(Planet)]
pub struct Venus;

#[derive(Planet)]
pub struct Earth;

#[derive(Planet)]
pub struct Mars;

#[derive(Planet)]
pub struct Jupiter;

#[derive(Planet)]
pub struct Saturn;

#[derive(Planet)]
pub struct Uranus;

#[derive(Planet)]
pub struct Neptune;
