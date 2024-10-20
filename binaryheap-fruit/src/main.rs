use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "External GitHub Lab: Generating Fruit Salad with Rust and BinaryHeap"
)]
struct Opts {
    #[clap(short, long)]
    count: Option<u32>,
}

struct Config {
    count: u32,
}

const FRUITS: [&str; 9] = [
    "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
];

fn get_config() -> Config {
    let opts: Opts = Opts::parse();
    Config {
        count: opts.count.unwrap_or(FRUITS.len() as u32),
    }
}

#[derive(Eq, PartialEq, Hash)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Greater,
            (Fruit::Other(a), Fruit::Other(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad(amount: usize) -> BinaryHeap<Fruit> {
    let amount = amount.clamp(1, FRUITS.len() + 1);
    let mut rng = thread_rng();
    let mut fruit_salad = BinaryHeap::new();

    while fruit_salad.len() < amount {
        let fruit = FRUITS.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn main() {
    let cfg = get_config();
    let fruit_salad = generate_fruit_salad(cfg.count as usize);

    // To track fruit generation statistics.
    let mut unique_fruit = HashMap::new();
    for fruit in fruit_salad.into_iter() {
        *unique_fruit.entry(fruit).or_insert(0) += 1;
    }

    let mut fruit: Vec<_> = unique_fruit.iter().collect();
    fruit.sort();
    println!("Random Fruit Salad With Two Servings of Figs:");
    for (fruit, count) in fruit.iter().rev() {
        match fruit {
            Fruit::Fig => println!("{}\tFig", count),
            Fruit::Other(fruit_name) => println!("{}\t{}", count, fruit_name),
        }
    }
}
