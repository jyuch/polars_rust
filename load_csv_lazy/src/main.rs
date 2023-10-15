use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let df = LazyCsvReader::new("data/load_csv_lazy.csv")
        .finish()?
        .select([col("value").sum()])
        .collect()?;

    println!("{}", df);

    Ok(())
}
