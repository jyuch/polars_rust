use chrono::Local;
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let part = LazyCsvReader::new("data/part.csv")
        .has_header(true)
        .with_schema(Some(Arc::new(schemata::part())))
        .finish()?;

    let purchase = LazyCsvReader::new("data/purchase.csv")
        .has_header(true)
        .with_schema(Some(Arc::new(schemata::purchase())))
        .finish()?;

    let now = Local::now().naive_local();

    let plan = purchase
        .clone()
        .join(
            part.clone(),
            [col("qoid"), col("ver")],
            [col("qoid"), col("ver")],
            JoinArgs::new(JoinType::Inner),
        )
        .select([
            col("qoid"),
            col("ver"),
            col("name"),
            col("unit_qty"),
            col("qty"),
            (col("unit_qty") * col("qty")).alias("total_qty"),
            col("valid_start"),
            col("valid_end"),
        ])
        .filter(col("valid_start").lt_eq(lit(now)))
        .filter(col("valid_end").gt(lit(now)));

    println!("{}", plan.explain(true)?);
    println!("{}", plan.collect()?);

    Ok(())
}
