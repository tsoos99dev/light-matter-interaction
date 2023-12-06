use std::f64::consts::PI;

use super::{grid::Grid, IMP0};

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
        let cezh = grid.cezh[self.location];
        let chye = grid.chye[self.location];

        let er = IMP0 / cezh;
        let mr = 1.0 / (IMP0 * chye);
        let n = f64::sqrt(er * mr);

        grid.hy[self.location - 1] +=
            -self.amplitude * f64::sin(2.0 * PI / self.wavelength * t) / super::IMP0 / mr;
        grid.ez[self.location] +=
            self.amplitude * f64::sin(2.0 * PI / self.wavelength * (t + 0.5 * (1.0 + n))) / n;
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
        let cezh = grid.cezh[self.location];
        let chye = grid.chye[self.location];

        let er = IMP0 / cezh;
        let mr = 1.0 / (IMP0 * chye);
        let n = f64::sqrt(er * mr);

        grid.hy[self.location - 1] += -self.amplitude
            * f64::exp(-(t - 3.0 * self.width).powi(2) / self.width.powi(2))
            / IMP0
            / mr;

        grid.ez[self.location] += self.amplitude
            * f64::exp(-(t + 0.5 * (1.0 + n) - 3.0 * self.width).powi(2) / self.width.powi(2))
            / n;
    }
}
