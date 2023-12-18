use std::{marker::PhantomData, vec};

use self::grid::Grid;

pub mod boundary;
pub mod export;
mod grid;
pub mod material;
pub mod probe;
pub mod source;

const IMP0: f64 = 377.0;

mod state {
    pub trait Boundary {}
    pub struct NoBoundary;
    pub struct WithBoudary;

    impl Boundary for NoBoundary {}
    impl Boundary for WithBoudary {}
}

pub struct FDTDSim<'a, Left: state::Boundary, Right: state::Boundary> {
    grid: Grid,
    xstep: usize,
    tstep: usize,
    sources: Vec<Box<dyn source::Source + 'a>>,
    materials: Vec<Box<dyn material::Material + 'a>>,
    probes: Vec<&'a mut dyn probe::Probe>,
    left_boundary_kind: boundary::BoundaryKind,
    right_boundary_kind: boundary::BoundaryKind,

    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
}

impl<'a, Left: state::Boundary, Right: state::Boundary> FDTDSim<'a, Left, Right> {
    pub fn add_source(mut self, source: impl source::Source + 'a) -> Self {
        self.sources.push(Box::new(source));
        self
    }

    pub fn add_material(mut self, material: impl material::Material + 'a) -> Self {
        self.materials.push(Box::new(material));
        self
    }

    pub fn add_probe(mut self, probe: &'a mut impl probe::Probe) -> Self {
        self.probes.push(probe);
        self
    }

    pub fn run(mut self) {
        for material in &mut self.materials {
            material.create(&mut self.grid);
        }

        let mut left_boundary: Box<dyn boundary::Boundary> = match self.left_boundary_kind {
            boundary::BoundaryKind::ABC => Box::new(boundary::LeftABC::new(&self.grid)),
            boundary::BoundaryKind::ABC2 => Box::new(boundary::LeftABC2::new(&self.grid)),
        };

        let mut right_boundary: Box<dyn boundary::Boundary> = match self.right_boundary_kind {
            boundary::BoundaryKind::ABC => Box::new(boundary::RightABC::new(&self.grid)),
            boundary::BoundaryKind::ABC2 => Box::new(boundary::RightABC2::new(&self.grid)),
        };

        for t in 0..self.tstep {
            for probe in &mut self.probes {
                probe.measure(&self.grid, t);
            }

            for i in 0..self.xstep - 1 {
                self.grid.hy[i] = self.grid.chyh[i] * self.grid.hy[i]
                    + self.grid.chye[i] * (self.grid.ez[i + 1] - self.grid.ez[i]);
            }

            for source in &self.sources {
                source.evaluate(&mut self.grid, t as f64)
            }

            for i in 1..self.xstep - 1 {
                self.grid.ez[i] = self.grid.ceze[i] * self.grid.ez[i]
                    + self.grid.cezh[i] * (self.grid.hy[i] - self.grid.hy[i - 1]);
            }

            for material in &mut self.materials {
                material.update(&mut self.grid, t as f64);
            }

            left_boundary.update(&mut self.grid);
            right_boundary.update(&mut self.grid);
        }
    }
}

impl<'a> FDTDSim<'a, state::NoBoundary, state::NoBoundary> {
    pub fn new(
        xstep: usize,
        tstep: usize,
        dx: f64,
    ) -> FDTDSim<'a, state::NoBoundary, state::NoBoundary> {
        let grid = grid::Grid {
            dx,
            ez: vec![0.0; xstep],
            hy: vec![0.0; xstep - 1],
            ceze: vec![1.0; xstep],
            cezh: vec![IMP0; xstep],
            chyh: vec![1.0; xstep - 1],
            chye: vec![1.0 / IMP0; xstep - 1],
        };

        FDTDSim {
            grid,
            xstep,
            tstep,
            sources: vec![],
            materials: vec![],
            probes: vec![],
            left_boundary_kind: boundary::BoundaryKind::ABC2,
            right_boundary_kind: boundary::BoundaryKind::ABC2,
            _left: Default::default(),
            _right: Default::default(),
        }
    }
}

impl<'a, Right: state::Boundary> FDTDSim<'a, state::NoBoundary, Right> {
    pub fn set_left_boundary(
        self,
        kind: boundary::BoundaryKind,
    ) -> FDTDSim<'a, state::WithBoudary, Right> {
        FDTDSim {
            grid: self.grid,
            xstep: self.xstep,
            tstep: self.tstep,
            sources: self.sources,
            materials: self.materials,
            probes: self.probes,
            left_boundary_kind: kind,
            right_boundary_kind: self.right_boundary_kind,
            _left: Default::default(),
            _right: Default::default(),
        }
    }
}

impl<'a, Left: state::Boundary> FDTDSim<'a, Left, state::NoBoundary> {
    pub fn set_right_boundary(
        self,
        kind: boundary::BoundaryKind,
    ) -> FDTDSim<'a, Left, state::WithBoudary> {
        FDTDSim {
            grid: self.grid,
            xstep: self.xstep,
            tstep: self.tstep,
            sources: self.sources,
            materials: self.materials,
            probes: self.probes,
            left_boundary_kind: self.left_boundary_kind,
            right_boundary_kind: kind,
            _left: Default::default(),
            _right: Default::default(),
        }
    }
}
