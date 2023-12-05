use std::{error::Error, f64::consts::PI};

const IMP0: f64 = 377.0;

use light_matter_interaction::fdtd;

fn main() {
    let mut probe_point = fdtd::probe::EPoint::new(120);

    fdtd::FDTDSim::new(200, 450)
        .add_source(fdtd::source::Harmonic::new(70.0, 50))
        .add_probe(&mut probe_point)
        .run();

    let _ = export_snapshot("out/sim.csv", &probe_point.data);
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

fn sim() -> Result<(), Box<dyn Error>> {
    const SIZE: usize = 200;
    const MAX_TIME: usize = 450;

    const LOSS: f64 = 0.0253146;

    const N_LAMBDA: f64 = 40.0;

    let mut ez: [f64; SIZE] = [0.0; SIZE];
    let mut hy: [f64; SIZE - 1] = [0.0; SIZE - 1];

    let mut ceze: [f64; SIZE] = [1.0; SIZE];
    let mut cezh: [f64; SIZE] = [IMP0; SIZE];
    let chyh: [f64; SIZE - 1] = [1.0; SIZE - 1];
    let chye: [f64; SIZE - 1] = [1.0 / IMP0; SIZE - 1];

    for i in 0..SIZE {
        if i >= 100 {
            ceze[i] = (1.0 - LOSS) / (1.0 + LOSS);
            cezh[i] = IMP0 / 4.0 / (1.0 + LOSS);
        }
    }

    let mut data: Vec<[f64; SIZE]> = Vec::with_capacity(MAX_TIME);

    let duration = 20;
    let frame_rate = 30;
    let frame_delay = usize::max(1, MAX_TIME / (duration * frame_rate));

    for t in 0..MAX_TIME {
        for i in 0..SIZE - 1 {
            hy[i] = chyh[i] * hy[i] + chye[i] * (ez[i + 1] - ez[i]);
        }

        // hy[49] += -f64::exp(-((t as i32 - 30).pow(2)) as f64 / 100.0) / IMP0;
        hy[49] += -f64::sin(2.0 * PI / N_LAMBDA * (t as i32) as f64) / IMP0;
        // ez[50] += f64::exp(-((t as i32 + 1 - 30).pow(2)) as f64 / 100.0);
        ez[50] += f64::sin(2.0 * PI / N_LAMBDA * (t as i32 + 1) as f64);

        ez[0] = ez[1];

        for i in 1..SIZE - 1 {
            ez[i] = ceze[i] * ez[i] + cezh[i] * (hy[i] - hy[i - 1]);
        }

        if t % frame_delay == 0 {
            data.push(ez.clone());
        }
    }

    let mut wtr = csv::Writer::from_path("sim.csv")?;
    for ez in &data {
        wtr.write_record(ez.map(|e| format!("{:.3}", e)))?;
    }

    wtr.flush()?;

    Ok(())
}
