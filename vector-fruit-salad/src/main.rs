/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
*/

use std::io::{BufRead, BufReader};

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruits = random_fruit(2);
    let end = "end";
    println!("Add the ingredients to the salad. To complete the entry, enter 'end'");
    loop {
        let input = read_stdin().trim().to_string().to_lowercase();
        if input == end {
            break;
        }
        fruits.push(input);
    }

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    println!("Fruit Salad: {}", fruits.join(", "));
}



fn random_fruit(count: u32) -> Vec<String> {
    if count == 0 {
        return Vec::new();
    }

    let fruits = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];
    if count as usize >= fruits.len() {
        return fruits.iter().map(|fruit| fruit.to_string()).collect();
    }

    let mut rng = thread_rng();
    let mut random_fruits: Vec<String> = Vec::new();
    let limit = if fruits.len() < count as usize {
        fruits.len()
    } else {
        count as usize
    };

    while random_fruits.len() < limit {
        let fruit = fruits.choose(&mut rng).unwrap();
        random_fruits.push(fruit.to_string());
    }

    random_fruits
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

#[cfg(test)]
mod tests {
    use super::_read_stdin;
    use std::io::Cursor;

    #[test]
    fn test_read_input() {
        let input = "Hello, world!\n";
        let expected_output = "Hello, world!";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_read_input_empty() {
        let input = "";
        let expected_output = "";
        let mut reader = Cursor::new(input);
        let output = _read_stdin(&mut reader);
        assert_eq!(output, expected_output);
    }
}
