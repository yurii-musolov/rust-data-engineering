use clap::Parser;
use fruit_salad_maker::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "Make a Fruit Salad"
)]
struct Opts {
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let line = std::fs::read_to_string(filename).expect("Could not read file");
            split_line(&line)
        }
        None => opts
            .fruits
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad);
}

fn split_line(line: &str) -> Vec<String> {
    line.split(',').map(|s| s.trim().to_string()).collect()
}
fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}
