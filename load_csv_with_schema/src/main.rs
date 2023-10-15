use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let schema = Schema::from_iter(vec![
        Field::new("qoid", DataType::Utf8),
        Field::new("ver", DataType::UInt32),
        Field::new("name", DataType::Utf8),
        Field::new("unit_qty", DataType::Float64),
        Field::new("is_purchase", DataType::Boolean),
        Field::new(
            "valid_start",
            DataType::Datetime(TimeUnit::Milliseconds, Some("Asia/Tokyo".into())),
        ),
        Field::new(
            "valid_end",
            DataType::Datetime(TimeUnit::Milliseconds, Some("Asia/Tokyo".into())),
        ),
    ]);

    let df = CsvReader::from_path("data/part.csv")?
        .has_header(true)
        .with_schema(Some(Arc::new(schema)))
        .finish()?;

    println!("{}", df);

    Ok(())
}
