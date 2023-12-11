use super::grid::Grid;

pub trait Probe {
    fn measure(&mut self, grid: &Grid, t: usize);
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
    fn measure(&mut self, grid: &Grid, _t: usize) {
        self.data.push(grid.ez[self.location]);
    }
}

pub struct EField {
    pub interval: usize,
    pub spacing: usize,
    pub data: Vec<Vec<f64>>,
}

impl EField {
    pub fn new(interval: usize, spacing: usize) -> EField {
        EField {
            interval,
            spacing,
            data: vec![],
        }
    }
}

impl Probe for EField {
    fn measure(&mut self, grid: &Grid, t: usize) {
        if t % self.interval == 0 {
            self.data
                .push(grid.ez.clone().into_iter().step_by(self.spacing).collect());
        }
    }
}
