use light_matter_interaction::fdtd::{self, export::export_sim};

fn main() {
    // let mut probe_point = fdtd::probe::EPoint::new(120);
    let mut probe_field = fdtd::probe::EField::new(50, 10);

    fdtd::FDTDSim::new(30000, 30000, 1e-10)
        .add_source(fdtd::source::Harmonic::new(1.0, 724.0, 20))
        // .add_source(fdtd::source::Harmonic::new(0.5, 2500.0, 20))
        // .add_source(fdtd::source::Harmonic::new(0.5, 3000.0, 20))
        // .add_source(fdtd::source::Gaussian::new(1.0, 200.0, 20))
        // .add_material(fdtd::material::BoundDipoleArray::new(
        //     1e-4,
        //     10000..30000,
        //     10,
        // ))
        .add_material(fdtd::material::Lorentz::new(
            1170.0 / f64::sqrt(5.0),
            2000.0,
            4800.0,
            1.0,
            10000..30000,
        ))
        // .add_material(fdtd::material::LosslessDielectric::new(4.0, 10000..25000))
        // .add_material(fdtd::material::LossyDielectric::new(
        // 4.0,
        // 0.0253146,
        // 100..200,
        // ))
        // .add_probe(&mut probe_point)
        .add_probe(&mut probe_field)
        .run();

    // let _ = export_snapshot("out/sim.csv", &probe_point.data);
    let _ = export_sim("out/sim.csv", &probe_field.data);
}
