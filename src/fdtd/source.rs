use std::f64::consts::PI;

use super::grid::Grid;

pub trait Source {
    fn evaluate(&self, grid: &mut Grid, t: f64);
}

pub struct Harmonic {
    amplitude: f64,
    wavelength: f64,
    location: usize,
}

impl Harmonic {
    pub fn new(amplitude: f64, wavelength: f64, location: usize) -> Harmonic {
        Harmonic {
            amplitude,
            wavelength,
            location,
        }
    }
}

impl Source for Harmonic {
    fn evaluate(&self, grid: &mut Grid, t: f64) {
        grid.hy[self.location - 1] +=
            -self.amplitude * f64::sin(2.0 * PI / self.wavelength * t) / super::IMP0;
        grid.ez[self.location] += self.amplitude * f64::sin(2.0 * PI / self.wavelength * (t + 1.0));
    }
}

pub struct Gaussian {
    amplitude: f64,
    width: f64,
    location: usize,
}

impl Gaussian {
    pub fn new(amplitude: f64, width: f64, location: usize) -> Gaussian {
        Gaussian {
            amplitude,
            width,
            location,
        }
    }
}

impl Source for Gaussian {
    fn evaluate(&self, grid: &mut Grid, t: f64) {
        grid.hy[self.location - 1] += -self.amplitude
            * f64::exp(-(t - 3.0 * self.width) * (t - 3.0 * self.width) / self.width.powi(2))
            / super::IMP0;
        grid.ez[self.location] += self.amplitude
            * f64::exp(
                -(t + 1.0 - 3.0 * self.width) * (t + 1.0 - 3.0 * self.width) / self.width.powi(2),
            );
    }
}
