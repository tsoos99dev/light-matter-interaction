use std::f64::consts::PI;

use super::grid::Grid;

pub trait Source {
    fn evaluate(&self, grid: &mut Grid, t: f64);
}

pub struct Harmonic {
    wavelength: f64,
    location: usize,
}

impl Harmonic {
    pub fn new(wavelength: f64, location: usize) -> Harmonic {
        Harmonic {
            wavelength,
            location,
        }
    }
}

impl Source for Harmonic {
    fn evaluate(&self, grid: &mut Grid, t: f64) {
        grid.hy[self.location - 1] += -f64::sin(2.0 * PI / self.wavelength * t) / super::IMP0;
        grid.ez[self.location] += f64::sin(2.0 * PI / self.wavelength * (t + 1.0) as f64);
    }
}
