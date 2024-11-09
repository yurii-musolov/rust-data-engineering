use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use clap::Parser;
use fruit_salad_maker::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "Make a Fruit Salad"
)]
struct Opts {
    #[clap(long)]
    fruits: Option<String>,
    #[clap(long)]
    input_file: Option<String>,
    #[clap(long)]
    output_file: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let fruit_list = match opts.input_file {
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

    match opts.output_file {
        Some(path) => {
            let content = fruit_salad.join(", ");
            write_to_file(&content, &path);
        }
        None => display_fruit_salad(fruit_salad),
    }
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

fn write_to_file(content: &str, path: &str) {
    let file = match File::create(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => panic!("File not found: {}", error),
            std::io::ErrorKind::PermissionDenied => {
                panic!("Insufficient permissions to open the file: {}", error)
            }
            _ => panic!("Error opening file: {}", error),
        },
    };

    let mut writer = BufWriter::new(file);
    match writer.write_all(content.as_bytes()) {
        io::Result::Ok(_) => println!("Write all bytes."),
        io::Result::Err(e) => panic!("Error write lines: {}", e),
    };
    let _ = writer.flush();
}
