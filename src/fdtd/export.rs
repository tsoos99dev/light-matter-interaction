use std::error::Error;

pub fn export_snapshot<'a>(
    path: &str,
    data: impl IntoIterator<Item = &'a f64>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(data.into_iter().map(|e| format!("{:.3}", e)))?;

    wtr.flush()?;

    Ok(())
}

pub fn export_sim<'a, Row, Data>(path: &str, data: Data) -> Result<(), Box<dyn Error>>
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
