use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let df = LazyCsvReader::new("data/single_integer.csv")
        .finish()?
        .select([col("value").sum()])
        .collect()?;

    println!("{}", df);

    Ok(())
}
