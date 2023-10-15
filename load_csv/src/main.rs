use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let df = CsvReader::from_path("data/load_csv.csv")?.finish()?;
    println!("{}", &df);

    let df = df.clone().lazy().select([col("value").sum()]).collect()?;
    println!("{}", df);

    Ok(())
}
