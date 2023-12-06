use light_matter_interaction::fdtd::{self, export::export_sim};

fn main() {
    let mut probe_point = fdtd::probe::EPoint::new(120);
    let mut probe_field = fdtd::probe::EField::new(2);

    fdtd::FDTDSim::new(200, 600)
        // .add_source(fdtd::source::Harmonic::new(1.0, 40.0, 20))
        .add_source(fdtd::source::Gaussian::new(1.0, 10.0, 50))
        .add_material(fdtd::material::LosslessDielectric::new(4.0, 100..200))
        // .add_material(fdtd::material::LossyDielectric::new(
        //     4.0,
        //     0.0253146,
        //     100..200,
        // ))
        .add_probe(&mut probe_point)
        .add_probe(&mut probe_field)
        .run();

    // let _ = export_snapshot("out/sim.csv", &probe_point.data);
    let _ = export_sim("out/sim.csv", &probe_field.data);
}
