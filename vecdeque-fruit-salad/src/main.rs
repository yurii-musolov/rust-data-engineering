use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

fn main() {
    let mut fruits: VecDeque<String> = VecDeque::new();
    fruits.push_back("Arbutus".to_string());
    fruits.push_back("Loquat".to_string());
    fruits.push_back("Strawberry Tree Berry".to_string());

    shuffle(&mut fruits);

    fruits.push_front("Pomegranate".to_string());
    fruits.push_back("Fig".to_string());
    fruits.push_back("Cherry".to_string());

    print_fruits(&fruits);
    print_random_fruit(&fruits);
    print_format_command();
    print_available_commands();

    loop {
        match read_command() {
            Some((command, fruit)) => match command.as_str() {
                "add_begin" => {
                    println!("Fruit: '{}' added to the begin of the queue", &fruit);
                    fruits.push_front(fruit);
                    print_fruits(&fruits);
                }
                "add_end" => {
                    println!("Fruit: '{}' added to the end of the queue", &fruit);
                    fruits.push_back(fruit);
                    print_fruits(&fruits);
                }
                "remove_begin" => {
                    println!("Fruit: '{}' removed from the begin of the queue", &fruit);
                    fruits.pop_front();
                    print_fruits(&fruits);
                }
                "remove_end" => {
                    println!("Fruit: '{}' removed from the end of the queue", &fruit);
                    fruits.pop_back();
                    print_fruits(&fruits);
                }
                "end" => {
                    print_fruits(&fruits);
                    break;
                }
                _ => {
                    println!("Not available command: {}", command);
                    print_available_commands();
                }
            },
            None => {
                print_format_command();
                continue;
            }
        };
    }
}

fn read_command() -> Option<(String, String)> {
    let input = read_stdin();
    let input: Vec<_> = input.split(":").into_iter().map(|s| s.trim()).collect();
    if input.len() == 0 {
        return None;
    }

    let command = input[0].to_lowercase();
    if command == "end" {
        return Some((String::from("end"), String::new()));
    }

    if input.len() != 2 {
        return None;
    }

    return Some((command, String::from(input[1])));
}

fn print_format_command() {
    println!("Example command: 'add_end: Cherry'")
}

fn print_available_commands() {
    println!("Available commands: `end`, `add_begin:fruit`, `add_end:fruit`, `remove_begin:fruit`, `remove_end:fruit`")
}

fn shuffle(fruits: &mut VecDeque<String>) {
    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);
}

fn print_random_fruit(fruits: &VecDeque<String>) {
    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);
    match fruits.choose(&mut rng) {
        Some(fruit) => println!("Random fruit: {}", *fruit),
        None => println!("Fruit queue is empty"),
    }
}

fn print_fruits(fruits: &VecDeque<String>) {
    println!(
        "Fruit queue : {}",
        fruits.clone().into_iter().collect::<Vec<_>>().join(", ")
    );
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
