use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "External GitHub Lab: Generating Unique Fruits with Rust and BTreeSet"
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
    "apple",
    "banana",
    "cherry",
    "date",
    "elderberry",
    "fig",
    "grape",
    "honeydew",
];

fn main() {
    let cfg = get_config();
    let amount = cfg.count as usize;

    let fruit_set = random_fruits(amount);

    let rev_fruit: Vec<_> = fruit_set.iter().rev().collect();
    println!("{}: {:?}", amount, rev_fruit);
}

fn random_fruits(amount: usize) -> BTreeSet<&'static str> {
    let amount = amount.clamp(0, FRUITS.len() + 1);
    let mut rng = thread_rng();

    let mut fruit_set = BTreeSet::new();
    let mut shuffled_fruits = FRUITS.clone();
    shuffled_fruits.shuffle(&mut rng);

    for fruit in shuffled_fruits {
        fruit_set.insert(fruit);
        if fruit_set.len() >= amount {
            break;
        }
    }

    fruit_set
}
