use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub const FRUITS: [&str; 10] = [
    "Arbutus",
    "Loquat",
    "Strawberry Tree Berry",
    "Pomegranate",
    "Fig",
    "Cherry",
    "Orange",
    "Pear",
    "Peach",
    "Apple",
];

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut rng = thread_rng();
    let mut fruits: Vec<_> = FRUITS.iter().map(|s| s.to_string()).collect();
    fruits.shuffle(&mut rng);

    let mut fruits: Vec<_> = fruits.into_iter().take(num_fruits).collect();
    fruits.sort();
    fruits
}
pub struct Components {
    v: Vec<String>,
}

impl Components {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    fn get_full_name(&self) -> String {
        String::new()
        // format!("{} {}", self.first_name, &self.last_name)
    }
}
