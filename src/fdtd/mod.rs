use std::{marker::PhantomData, vec};

mod grid;
pub mod material;
pub mod probe;
pub mod source;

const IMP0: f64 = 377.0;

mod state {
    pub trait State {}
    pub struct Empty;
    pub struct WithLeftBoudary;
    pub struct WithRightBoudary;

    impl State for Empty {}
    impl State for WithLeftBoudary {}
    impl State for WithRightBoudary {}

    pub trait WithoutLeftBoundary: State {}

    impl WithoutLeftBoundary for Empty {}
    impl WithoutLeftBoundary for WithRightBoudary {}

    pub trait WithoutRightBoundary: State {}

    impl WithoutRightBoundary for Empty {}
    impl WithoutRightBoundary for WithLeftBoudary {}
}

pub struct FDTDSim<'a, S: state::State> {
    xstep: usize,
    tstep: usize,
    sources: Vec<Box<dyn source::Source + 'a>>,
    materials: Vec<Box<dyn material::Material + 'a>>,
    probes: Vec<&'a mut dyn probe::Probe>,

    _p: PhantomData<S>,
}

impl<'a, S: state::State> FDTDSim<'a, S> {
    pub fn run(mut self) -> FDTDSim<'a, S> {
        let mut grid = grid::Grid {
            ez: vec![0.0; self.xstep],
            hy: vec![0.0; self.xstep],
            ceze: vec![1.0; self.xstep],
            cezh: vec![IMP0; self.xstep],
            chyh: vec![1.0; self.xstep - 1],
            chye: vec![1.0 / IMP0; self.xstep - 1],
        };

        for material in &self.materials {
            material.create(&mut grid);
        }

        for t in 0..self.tstep {
            for probe in &mut self.probes {
                probe.measure(&grid, t);
            }

            for i in 0..self.xstep - 1 {
                grid.hy[i] =
                    grid.chyh[i] * grid.hy[i] + grid.chye[i] * (grid.ez[i + 1] - grid.ez[i]);
            }

            for source in &self.sources {
                source.evaluate(&mut grid, t as f64)
            }

            grid.ez[0] = grid.ez[1];

            for i in 1..self.xstep - 1 {
                grid.ez[i] =
                    grid.ceze[i] * grid.ez[i] + grid.cezh[i] * (grid.hy[i] - grid.hy[i - 1]);
            }
        }

        self
    }
}

impl<'a> FDTDSim<'a, state::Empty> {
    pub fn new(xstep: usize, tstep: usize) -> FDTDSim<'a, state::Empty> {
        FDTDSim {
            xstep,
            tstep,
            sources: vec![],
            materials: vec![],
            probes: vec![],
            _p: Default::default(),
        }
    }

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
}

// impl<S: sim_builder::WithoutLeftBoundary> SimBuilder<S> {
//     pub fn set_left_boundary(self) -> SimBuilder<sim_builder::WithLeftBoudary> {
//         todo!()
//     }
// }

// impl<S: sim_builder::WithoutRightBoundary> SimBuilder<S> {
//     pub fn set_right_boundary(self) -> SimBuilder<sim_builder::WithRightBoudary> {
//         todo!()
//     }
// }
