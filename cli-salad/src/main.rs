use clap::Parser;
use cli_salad::{create_fruit_salad, FRUITS};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    // #[clap(short, long, value_delimiter(','), required(false))]
    // number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits = opts.number;
    if num_fruits == 0 || num_fruits > FRUITS.len() {
        println!(
            "The value must be greater than 0 and less than {}",
            FRUITS.len()
        );
        return;
    }

    let salad = create_fruit_salad(num_fruits);
    println!(
        "Created fruit salad with {} fruits: {:?}",
        num_fruits, salad
    );
}
