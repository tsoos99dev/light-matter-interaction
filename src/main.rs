use std::error::Error;

const IMP0: f64 = 377.0;

use light_matter_interaction::fdtd;

fn main() {
    let mut probe_point = fdtd::probe::EPoint::new(120);
    let mut probe_field = fdtd::probe::EField::new(1);

    fdtd::FDTDSim::new(200, 1000)
        // .add_source(fdtd::source::Harmonic::new(1.0, 40.0, 50))
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

fn export_snapshot<'a>(
    path: &str,
    data: impl IntoIterator<Item = &'a f64>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(data.into_iter().map(|e| format!("{:.3}", e)))?;

    wtr.flush()?;

    Ok(())
}

fn export_sim<'a, Row, Data>(path: &str, data: Data) -> Result<(), Box<dyn Error>>
where
    Row: IntoIterator<Item = &'a f64>,
    Data: IntoIterator<Item = Row>,
{
    let mut wtr = csv::Writer::from_path(path)?;
    for row in data.into_iter() {
        wtr.write_record(row.into_iter().map(|e| format!("{:.3}", e)))?;
    }

    wtr.flush()?;

    Ok(())
}
