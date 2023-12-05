use super::grid::Grid;

pub trait Probe {
    fn measure(&mut self, grid: &Grid);
}

pub struct EPoint {
    pub location: usize,
    pub data: Vec<f64>,
}

impl EPoint {
    pub fn new(location: usize) -> EPoint {
        EPoint {
            location,
            data: vec![],
        }
    }
}

impl Probe for EPoint {
    fn measure(&mut self, grid: &Grid) {
        self.data.push(grid.ez[self.location]);
    }
}
