use super::{grid::Grid, IMP0};
use std::ops::Range;

pub trait Material {
    fn create(&self, grid: &mut Grid);
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
            grid.cezh[i] = IMP0 / self.er;
        }
    }
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
}
