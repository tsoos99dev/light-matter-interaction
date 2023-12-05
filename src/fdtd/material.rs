use super::grid::Grid;

pub trait Material {
    fn create(&self, grid: &mut Grid);
}

pub struct LosslessDielectric {}

impl Material for LosslessDielectric {
    fn create(&self, grid: &mut Grid) {}
}
