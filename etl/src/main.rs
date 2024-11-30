use clap::Parser;
use serde::Serialize;
use std::error::Error;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "Apply ETL concepts by enhancing a simple data processing pipeline in Rust"
)]
struct Opts {
    #[clap(short, long)]
    file: Option<String>,
}

#[derive(Debug)]
struct RawData {
    id: u32,
    value: i32,
}

#[derive(Debug, Serialize)]
struct CleanData {
    id: u32,
    value: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let raw = get_dataset();

    let cleaned = extract_transform_load(raw);

    let (total, average, max) = statistic(&cleaned);
    println!("Cleaned series. Total: {total}, average: {average}, max: {max}",);

    match opts.file {
        Some(path) => match csv::Writer::from_path(path) {
            Ok(mut wtr) => {
                cleaned.iter().for_each(|row| {
                    let _ = wtr.serialize(row);
                });
                wtr.flush()?;
            }
            Err(err) => println!("{err}"),
        },
        None => println!(
            "Slice cleaned data [0..2]: {:?}",
            cleaned[0..2].into_iter().collect::<Vec<_>>()
        ),
    }
    Ok(())
}

fn statistic(series: &Vec<CleanData>) -> (f32, f32, f32) {
    assert!(series.len() > 0);

    let mut total = 0.0;
    let mut max = 0.0;
    series.iter().for_each(|c| {
        total += c.value;
        if c.value > max {
            max = c.value
        }
    });

    (total, total / (series.len() as f32), max)
}

// Perform ETL process
fn extract_transform_load(raw: Vec<RawData>) -> Vec<CleanData> {
    let normalize = make_normalize(10.0); // Scale (value 10.0 as 100%).
    let is_valid = make_is_valid(-10.0, 110.0); // +-10%
    let range = make_range(0.0, 100.0);

    raw.into_iter()
        .map(|r| CleanData {
            id: r.id,
            value: normalize(r.value as f32),
        })
        .filter(|c| is_valid(c.value))
        .map(|c| CleanData {
            id: c.id,
            value: range(c.value),
        })
        .collect()
}

fn make_normalize(limit: f32) -> impl Fn(f32) -> f32 {
    let k = 100.0_f32 / limit;
    move |n| n * k
}

fn make_range(min: f32, max: f32) -> impl Fn(f32) -> f32 {
    move |n| n.min(max).max(min)
}

fn make_is_valid(min: f32, max: f32) -> impl Fn(f32) -> bool {
    move |n| n >= min && n <= max
}

fn get_dataset() -> Vec<RawData> {
    vec![
        RawData { id: 1, value: 10 },
        RawData { id: 2, value: -5 },
        RawData { id: 3, value: 14 },
        RawData { id: 4, value: 8 },
        RawData { id: 5, value: 4 },
        RawData { id: 6, value: 5 },
        RawData { id: 7, value: 6 },
        RawData { id: 8, value: 7 },
        RawData { id: 9, value: 4 },
        RawData { id: 10, value: 15 },
        RawData { id: 11, value: 7 },
        RawData { id: 12, value: 8 },
        RawData { id: 13, value: 6 },
        RawData { id: 14, value: 5 },
        RawData { id: 15, value: 2 },
        RawData { id: 16, value: -8 },
        RawData { id: 17, value: -1 },
        RawData { id: 18, value: 3 },
        RawData { id: 19, value: 4 },
        RawData { id: 20, value: 6 },
        RawData { id: 21, value: 5 },
        RawData { id: 22, value: 7 },
        RawData { id: 23, value: 6 },
    ]
}
