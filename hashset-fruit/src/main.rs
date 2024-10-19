use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "External GitHub Lab: Generating Unique Fruits with Rust and HashSet"
)]
struct Opts {
    #[clap(short, long)]
    count: Option<u32>,
}

struct Config {
    count: u32,
}

fn get_config() -> Config {
    let opts: Opts = Opts::parse();
    Config {
        count: opts.count.unwrap_or(FRUITS.len() as u32),
    }
}

const FRUITS: [&str; 8] = [
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elderberry",
    "Fig",
    "Grape",
    "Honeydew",
];

fn generate_fruit() -> &'static str {
    let mut rng = thread_rng();
    FRUITS.choose(&mut rng).unwrap()
}

fn main() {
    let cfg = get_config();

    let mut fruit: Vec<_> = Vec::with_capacity(cfg.count as usize);
    for _ in 0..cfg.count {
        fruit.push(generate_fruit());
    }
    println!("Generating {} random fruits...", fruit.len());

    // To track fruit generation statistics.
    let mut unique_fruit = HashMap::new();
    for fruit in fruit.into_iter() {
        *unique_fruit.entry(fruit).or_insert(1) += 1;
    }
    println!("Number of unique fruits generated: {}", unique_fruit.len());
}
