use clap::Parser;
use rasciigraph::{plot, Config};
use serde::Deserialize;
use std::fs::File;

const DEFAULT_FILE: &str = "./server_payment.csv";

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "Plotting data from a file.csv on a graph in the console"
)]
struct Opts {
    #[clap(short, long)]
    file: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Record {
    month: String,
    total: f64,
}

fn main() {
    let opts: Opts = Opts::parse();
    let filename = opts.file.unwrap_or(DEFAULT_FILE.to_string());
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };
    let mut rdr = csv::Reader::from_reader(file);

    let mut months = Vec::new();
    let mut totals = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = match result {
            Ok(record) => record,
            Err(err) => panic!("{}", err),
        };
        months.push(record.month);
        totals.push(record.total);
    }

    let histogram = plot(
        totals,
        Config::default()
            .with_offset(4)
            .with_height(8)
            .with_width(40)
            .with_caption("Payment for the server (EUR)".to_string()),
    );
    println!("{}", months.join(" > "));
    println!("{}", histogram);
}
