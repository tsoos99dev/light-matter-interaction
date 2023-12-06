use super::grid::Grid;

pub trait Boundary {
    fn update(&mut self, grid: &mut Grid);
}

pub enum BoundaryKind {
    ABC,
    ABC2,
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
        let size = grid.ez.len();
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

pub struct LeftABC2 {
    old_ez1: [f64; 3],
    old_ez2: [f64; 3],
    coeff: [f64; 3],
}

impl LeftABC2 {
    pub fn new(grid: &Grid) -> LeftABC2 {
        let size = grid.ez.len();
        let cezh0 = grid.cezh[size - 1];
        let chye = grid.chye[size - 2];
        let temp1 = f64::sqrt(cezh0 * chye);
        let temp2 = 1.0 / temp1 + 2.0 + temp1;
        let coeff = [
            -(1.0 / temp1 - 2.0 + temp1) / temp2,
            -2.0 * (temp1 - 1.0 / temp1) / temp2,
            4.0 * (temp1 + 1.0 / temp1) / temp2,
        ];

        LeftABC2 {
            old_ez1: [0.0; 3],
            old_ez2: [0.0; 3],
            coeff,
        }
    }
}

impl Boundary for LeftABC2 {
    fn update(&mut self, grid: &mut Grid) {
        let ezleft1 = grid.ez[1];
        let ezleft2 = grid.ez[2];

        grid.ez[0] = self.coeff[0] * (ezleft2 + self.old_ez2[0])
            + self.coeff[1] * (self.old_ez1[0] + self.old_ez1[2] - ezleft1 - self.old_ez2[1])
            + self.coeff[2] * self.old_ez1[1]
            - self.old_ez2[2];

        self.old_ez2 = self.old_ez1.clone();
        self.old_ez1.clone_from_slice(&grid.ez[0..3]);
    }
}

pub struct RightABC2 {
    old_ez1: [f64; 3],
    old_ez2: [f64; 3],
    coeff: [f64; 3],
}

impl RightABC2 {
    pub fn new(grid: &Grid) -> RightABC2 {
        let size = grid.ez.len();
        let cezh0 = grid.cezh[size - 1];
        let chye = grid.chye[size - 2];
        let temp1 = f64::sqrt(cezh0 * chye);
        let temp2 = 1.0 / temp1 + 2.0 + temp1;
        let coeff = [
            -(1.0 / temp1 - 2.0 + temp1) / temp2,
            -2.0 * (temp1 - 1.0 / temp1) / temp2,
            4.0 * (temp1 + 1.0 / temp1) / temp2,
        ];

        RightABC2 {
            old_ez1: [0.0; 3],
            old_ez2: [0.0; 3],
            coeff,
        }
    }
}

impl Boundary for RightABC2 {
    fn update(&mut self, grid: &mut Grid) {
        let size = grid.ez.len();
        let ezright1 = grid.ez[size - 2];
        let ezright2 = grid.ez[size - 3];

        grid.ez[size - 1] = self.coeff[0] * (ezright2 + self.old_ez2[0])
            + self.coeff[1] * (self.old_ez1[0] + self.old_ez1[2] - ezright1 - self.old_ez2[1])
            + self.coeff[2] * self.old_ez1[1]
            - self.old_ez2[2];

        self.old_ez2 = self.old_ez1.clone();
        self.old_ez1.clone_from_slice(&grid.ez[size - 3..size]);
        self.old_ez1.reverse();
    }
}
