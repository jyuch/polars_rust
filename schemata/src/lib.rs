use polars::prelude::*;

pub fn part() -> Schema {
    Schema::from_iter(vec![
        Field::new("qoid", DataType::Utf8),
        Field::new("ver", DataType::UInt32),
        Field::new("name", DataType::Utf8),
        Field::new("unit_qty", DataType::Float64),
        Field::new("is_purchase", DataType::Boolean),
        Field::new(
            "valid_start",
            DataType::Datetime(TimeUnit::Milliseconds, None),
        ),
        Field::new(
            "valid_end",
            DataType::Datetime(TimeUnit::Milliseconds, None),
        ),
    ])
}

pub fn purchase() -> Schema {
    Schema::from_iter(vec![
        Field::new("qoid", DataType::Utf8),
        Field::new("ver", DataType::UInt32),
        Field::new("qty", DataType::Float64),
        Field::new(
            "purchase_date",
            DataType::Datetime(TimeUnit::Milliseconds, Some("Asia/Tokyo".into())),
        ),
    ])
}
