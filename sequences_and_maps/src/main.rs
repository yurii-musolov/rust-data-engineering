use chrono::{Datelike, Utc};
use std::collections::HashMap;

fn main() {
    example_vec();
    example_fruit_salad();
    example_vec_dequeue();
    example_hash_map();
    example_hashmap_count();
    example_hashmap_language();
}

fn example_vec() {
    println!("\n\tExample: std::vec::Vec");
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.push("orange");
    println!("Fruit vector: {:?}", fruits);
}

fn example_fruit_salad() {
    println!("\n\tExample: fruit salad");
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);
    let salad = fruit.join(", ");
    println!("Fruit Salad: {salad}");
}

fn example_vec_dequeue() {
    println!("\n\tExample: std::collections::VecDeque");
    use std::collections::VecDeque;
    let mut fruit_deque = VecDeque::new();
    fruit_deque.push_back("apple");
    fruit_deque.push_front("cherry");
    println!("Fruit deque: {:?}", fruit_deque);
}

fn example_hash_map() {
    println!("\n\tExample: std::collections::HashMap");
    use std::collections::HashMap;
    let mut fruit_calories = HashMap::new();
    fruit_calories.insert("apple", 95);
    println!("Apple calories: {}", fruit_calories["apple"]);
}

fn example_hashmap_count() {
    println!("\n\tExample: HashMap count");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn example_hashmap_language() {
    println!("\n\tExample: hashmap-language");
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    let current_year = Utc::now().year();
    for year in years_active.values_mut() {
        *year = current_year - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;
        weights.insert(language.to_string(), weight);
    }

    weights
}
