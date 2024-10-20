use clap::Parser;
use config::Config as AppConfig;
use std::collections::HashMap;
use textwrap::fill;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yurii Musolov <yurii.musolov@google.com>",
    about = "External GitHub Lab: PageRank Algorithm in Rust"
)]
struct Opts {
    #[clap(short, long)]
    file: Option<String>,
}

struct Config {
    file: String,
}

fn get_config() -> Config {
    let opts: Opts = Opts::parse();
    Config {
        file: opts.file.unwrap_or(String::from("nba.json")),
    }
}

struct PageRank {
    damping: f64,      // Damping factor.
    iterations: usize, // Number of iterations.
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        let n = graph.len();
        let mut ranks = vec![1.0 / (n as f64); n];

        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; n];

            for (node, edges) in graph.iter().enumerate() {
                let contribution = ranks[node] / (edges.len() as f64);

                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            ranks = new_ranks;
        }

        normalize(&mut ranks);

        ranks
    }
}

fn main() {
    let cfg = get_config();
    let (names, graph) = get_ranks(&cfg.file);
    let pagerank = PageRank::new(0.85, 100);

    let ranks = pagerank.rank(&graph);

    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";

    println!("{}", fill(explanation, 78));
}

fn get_ranks(path: &str) -> (Vec<String>, Vec<Vec<usize>>) {
    let cfg = AppConfig::builder()
        .add_source(config::File::with_name(path))
        .build()
        .unwrap();
    let relations = cfg
        .try_deserialize::<HashMap<String, Vec<String>>>()
        .unwrap();

    let names: Vec<String> = relations.keys().into_iter().map(|s| s.clone()).collect();
    let mut ranks: Vec<Vec<usize>> = Vec::with_capacity(relations.len());
    let mut index: HashMap<String, usize> = HashMap::with_capacity(relations.len());
    names.iter().enumerate().for_each(|(idx, name)| {
        index.insert(name.clone(), idx);
    });

    names.iter().for_each(|name| {
        match relations.get(name) {
            Some(names) => {
                let mut indexes: Vec<usize> = Vec::with_capacity(names.len());
                names.iter().for_each(|relation| match index.get(relation) {
                    Some(i) => indexes.push(*i),
                    None => panic!("Incorrect relation \"{}\": [{}].", name, relation),
                });
                ranks.push(indexes);
            }
            None => panic!("Logically unreachable code."),
        };
    });

    (names, ranks)
}

fn normalize(ranks: &mut Vec<f64>) {
    let sum: f64 = ranks.iter().sum();
    ranks.iter_mut().for_each(|n| *n /= sum);
}
