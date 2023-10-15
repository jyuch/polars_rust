use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let df = CsvReader::from_path("data/single_integer.csv")?.finish()?;
    println!("{}", &df);

    let df = df.clone().lazy().select([col("value").sum()]).collect()?;
    println!("{}", df);

    Ok(())
}
