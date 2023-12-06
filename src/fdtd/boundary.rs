use super::grid::Grid;

pub trait Boundary {
    fn update(&mut self, grid: &mut Grid);
}

pub enum BoundaryKind {
    ABC,
}

pub struct LeftABC {
    old_ez: f64,
    coeff: f64,
}

impl LeftABC {
    pub fn new(grid: &Grid) -> LeftABC {
        let cezh0 = grid.cezh[0];
        let chye = grid.chye[0];
        let temp = f64::sqrt(cezh0 * chye);
        let coeff = (temp - 1.0) / (1.0 + temp);

        LeftABC { old_ez: 0.0, coeff }
    }
}

impl Boundary for LeftABC {
    fn update(&mut self, grid: &mut Grid) {
        let ez0 = grid.ez[0];
        let ezleft = grid.ez[1];

        grid.ez[0] = self.old_ez + self.coeff * (ezleft - ez0);
        self.old_ez = ezleft;
    }
}

pub struct RightABC {
    old_ez: f64,
    coeff: f64,
}

impl RightABC {
    pub fn new(grid: &Grid) -> RightABC {
        let size = 200;
        let cezh0 = grid.cezh[size - 1];
        let chye = grid.chye[size - 2];
        let temp = f64::sqrt(cezh0 * chye);
        let coeff = (temp - 1.0) / (1.0 + temp);

        RightABC { old_ez: 0.0, coeff }
    }
}

impl Boundary for RightABC {
    fn update(&mut self, grid: &mut Grid) {
        let size = grid.ez.len();
        let ez0 = grid.ez[size - 1];
        let ezright = grid.ez[size - 2];

        grid.ez[size - 1] = self.old_ez + self.coeff * (ezright - ez0);
        self.old_ez = ezright;
    }
}
