use super::{grid::Grid, IMP0};
use std::{f64::consts::PI, ops::Range};

pub trait Material {
    fn create(&mut self, grid: &mut Grid);

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
    fn create(&mut self, grid: &mut Grid) {
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
    fn create(&mut self, grid: &mut Grid) {
        for i in self.extent.clone() {
            grid.ceze[i] = (1.0 - self.loss) / (1.0 + self.loss);
            grid.cezh[i] = IMP0 / self.er / (1.0 + self.loss);
        }
    }

    fn update(&mut self, _grid: &mut Grid, _t: f64) {}
}

pub struct Lorentz {
    np: f64,
    n0: f64,
    nt: f64,
    eps_inf: f64,
    extent: Range<usize>,
    jp: Vec<f64>,
    ji: Vec<f64>,
    ezold: Vec<f64>,
    cjj: f64,
    cji: f64,
    cje: f64,
}

impl Lorentz {
    pub fn new(np: f64, n0: f64, nt: f64, eps_inf: f64, extent: Range<usize>) -> Lorentz {
        Lorentz {
            np,
            n0,
            nt,
            eps_inf,
            extent,
            jp: vec![],
            ji: vec![],
            ezold: vec![],
            cji: 0.0,
            cjj: 0.0,
            cje: 0.0,
        }
    }
}

impl Material for Lorentz {
    fn create(&mut self, grid: &mut Grid) {
        self.jp.resize(self.extent.len(), 0.0);
        self.ji.resize(self.extent.len(), 0.0);
        self.ezold.resize(grid.ez.len(), 0.0);

        self.cjj = (1.0 - 1.0 / self.nt) / (1.0 + 1.0 / self.nt);
        self.cje = 2.0 * PI.powi(2) / (IMP0 * self.np.powi(2)) / (1.0 + 1.0 / self.nt);
        self.cji = 2.0 * PI.powi(2) / self.n0.powi(2) / (1.0 + 1.0 / self.nt);

        dbg!(self.cjj);
        dbg!(self.cje);
        dbg!(self.cji);

        let temp = self.cje * IMP0 / (2.0 * self.eps_inf);
        let ceze = (1.0 - temp) / (1.0 + temp);
        let cezh = IMP0 / self.eps_inf / (1.0 + temp);

        println!("{} {}", ceze, cezh);

        for l in self.extent.clone() {
            grid.ceze[l] = ceze;
            grid.cezh[l] = cezh;
        }
    }

    fn update(&mut self, grid: &mut Grid, _t: f64) {
        let jitemp = self.ji.clone();

        for (i, v) in &mut self.ji.iter_mut().enumerate() {
            *v = *v + self.jp[i];
        }

        for (i, l) in self.extent.clone().step_by(5).enumerate() {
            grid.ez[l] -=
                0.5 * (1.0 + self.cjj) * IMP0 / (1.0 + self.cje * IMP0 / 2.0) * self.jp[i];
        }

        for (i, l) in self.extent.clone().step_by(5).enumerate() {
            self.jp[i] = self.cjj * self.jp[i] - self.cji * (self.ji[i] + jitemp[i])
                + self.cje * (grid.ez[l] + self.ezold[l]);
        }

        self.ezold = grid.ez.clone();
    }
}
