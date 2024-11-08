use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expects one argument (a string of words)");
        return;
    }

    let line = args[1].to_string();
    println!("The line contains {} words.", word_count(&line));
}

pub fn word_count(line: &str) -> u64 {
    let mut word_counter = WordCounter::new();
    line.split(" ")
        .into_iter()
        .map(|word| word.trim() /* TODO: remove [,.!?:;] */)
        .filter(|word| word.len() > 0)
        .for_each(|word| word_counter.add_word(word));
    word_counter.get_count()
}

pub fn word_frequencies<'a>(line: &'a str) -> Vec<(&'a str, u64)> {
    let mut word_counter = WordCounter::new();
    line.split(" ")
        .into_iter()
        .map(|word| word.trim() /* TODO: remove [,.!?:;] */)
        .filter(|word| word.len() > 0)
        .for_each(|word| word_counter.add_word(word));
    word_counter.get_frequencies()
}

pub struct WordCounter<'a> {
    frequencies: HashMap<&'a str, u64>,
}

impl<'a> WordCounter<'a> {
    pub fn new() -> Self {
        Self {
            frequencies: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.frequencies = HashMap::new();
    }

    pub fn add_word(&mut self, word: &'a str) {
        *self.frequencies.entry(word).or_insert(0) += 1;
    }

    pub fn get_count(&self) -> u64 {
        self.frequencies.values().into_iter().sum()
    }

    pub fn get_frequencies(&self) -> Vec<(&'a str, u64)> {
        let mut list: Vec<_> = self.frequencies.clone().into_iter().collect();
        list.sort_by(|(_, v1), (_, v2)| v2.partial_cmp(v1).unwrap());
        list
    }
}
