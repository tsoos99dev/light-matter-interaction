use super::{grid::Grid, IMP0};
use std::ops::Range;

pub trait Material {
    fn create(&self, grid: &mut Grid);

    fn update(&mut self, grid: &mut Grid, t: f64);
}

pub struct LosslessDielectric {
    er: f64,
    extent: Range<usize>,
}

impl LosslessDielectric {
    pub fn new(er: f64, extent: Range<usize>) -> LosslessDielectric {
        LosslessDielectric { er, extent }
    }
}

impl Material for LosslessDielectric {
    fn create(&self, grid: &mut Grid) {
        for i in self.extent.clone() {
            grid.ceze[i] = 1.0;
            grid.cezh[i] = IMP0 / self.er;
        }
    }

    fn update(&mut self, _grid: &mut Grid, _t: f64) {}
}

pub struct LossyDielectric {
    er: f64,
    loss: f64,
    extent: Range<usize>,
}

impl LossyDielectric {
    pub fn new(er: f64, loss: f64, extent: Range<usize>) -> LossyDielectric {
        LossyDielectric { er, loss, extent }
    }
}

impl Material for LossyDielectric {
    fn create(&self, grid: &mut Grid) {
        for i in self.extent.clone() {
            grid.ceze[i] = (1.0 - self.loss) / (1.0 + self.loss);
            grid.cezh[i] = IMP0 / self.er / (1.0 + self.loss);
        }
    }

    fn update(&mut self, _grid: &mut Grid, _t: f64) {}
}

pub struct SimpleBoundDipole {
    intrinsic_electric_field: f64,
    location: usize,
}

impl SimpleBoundDipole {
    pub fn new(intrinsic_electric_field: f64, location: usize) -> SimpleBoundDipole {
        SimpleBoundDipole {
            intrinsic_electric_field,
            location,
        }
    }
}

impl Material for SimpleBoundDipole {
    fn create(&self, _grid: &mut Grid) {}

    fn update(&mut self, grid: &mut Grid, _t: f64) {
        grid.ez[self.location] += -self.intrinsic_electric_field * grid.ez[self.location];
    }
}

pub struct BoundDipoleArray {
    intrinsic_electric_field: f64,
    extent: Range<usize>,
    spacing: usize,
}

impl BoundDipoleArray {
    pub fn new(
        intrinsic_electric_field: f64,
        extent: Range<usize>,
        spacing: usize,
    ) -> BoundDipoleArray {
        BoundDipoleArray {
            intrinsic_electric_field,
            extent,
            spacing,
        }
    }
}

impl Material for BoundDipoleArray {
    fn create(&self, _grid: &mut Grid) {}

    fn update(&mut self, grid: &mut Grid, _t: f64) {
        for location in self.extent.clone().step_by(self.spacing) {
            grid.hy[location - 1] += -self.intrinsic_electric_field * grid.hy[location - 1] / IMP0;
            grid.ez[location] += -self.intrinsic_electric_field * grid.ez[location];
            grid.hy[location] += -self.intrinsic_electric_field * grid.hy[location] / IMP0;
        }
    }
}
