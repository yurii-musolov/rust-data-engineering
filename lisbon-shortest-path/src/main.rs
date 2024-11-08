use std::io::{BufRead, BufReader};

use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let points = vec![
        "Belem Tower",
        "Jerónimos Monastery",
        "LX Factory",
        "Commerce Square",
        "Lisbon Cathedral",
    ];
    let nodes: Vec<_> = points.iter().map(|point| graph.add_node(&point)).collect();

    let belem_tower = nodes[0];
    let monastery = nodes[1];
    let lx_factory = nodes[2];
    let commerce_square = nodes[3];
    let lisbon_cathedral = nodes[4];

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    loop {
        println!("\n\nEnter start index and end index or 'exit' to terminate");
        println!(
            "Template <start_index, end_index>: {}, {}",
            0,
            points.len() - 1
        );
        for (i, point) in points.iter().enumerate() {
            println!("[{}]: {}", i, point);
        }

        match read_command(points.len()) {
            Ok((start, end)) => {
                let node_map = dijkstra(&graph, nodes[start], Some(nodes[end]), |e| *e.weight());

                if let Some(distance) = node_map.get(&lisbon_cathedral) {
                    println!(
                        "The shortest distance from Belem Tower to Lisbon Cathedral is {} km",
                        distance
                    );
                } else {
                    println!("No route found from Belem Tower to Lisbon Cathedral.");
                }

                continue;
            }
            Err(exit) => {
                if exit {
                    break;
                } else {
                    continue;
                }
            }
        }
    }
}

fn read_command(length: usize) -> Result<(usize, usize), bool> {
    let input = read_stdin();

    if input.trim() == "exit" {
        return Err(true);
    }

    let input: Vec<&str> = input.split(",").into_iter().map(|s| s.trim()).collect();
    if input.len() != 2 {
        return Err(false);
    }

    let start = match input[0].parse() {
        Ok(start) => start,
        Err(_) => return Err(false),
    };

    if start >= length {
        return Err(false);
    }

    let end = match input[1].parse() {
        Ok(end) => end,
        Err(_) => return Err(false),
    };

    if end >= length {
        return Err(false);
    }

    return Ok((start, end));
}

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}
