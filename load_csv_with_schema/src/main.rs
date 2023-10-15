use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let df = CsvReader::from_path("data/part.csv")?
        .has_header(true)
        .with_schema(Some(Arc::new(schemata::part())))
        .finish()?;

    println!("{}", df);

    Ok(())
}
