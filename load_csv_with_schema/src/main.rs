use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let schema = Schema::from_iter(vec![
        Field::new("col1", DataType::Utf8),
        Field::new("col2", DataType::Int64),
        Field::new("col3", DataType::Utf8),
        Field::new(
            "col4",
            DataType::Datetime(TimeUnit::Milliseconds, Some("Asia/Tokyo".into())),
        ),
        Field::new("col5", DataType::Float64),
    ]);

    let df = CsvReader::from_path("data/load_csv_with_schema.csv")?
        .has_header(true)
        .with_schema(Some(Arc::new(schema)))
        .finish()?;

    println!("{}", df);

    Ok(())
}
