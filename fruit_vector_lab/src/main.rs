use std::collections::HashMap;

fn main() {
    let mut fruit_salad = vec!["apple", "banana", "cherry"];
    fruit_salad.sort();
    fruit_salad.iter().for_each(|fruit| println!("{}", fruit));

    println!("Add 'pineapple' to salad.");
    fruit_salad.push("pineapple");
    fruit_salad.push("pineapple");
    fruit_salad.sort();
    print_statistic(&fruit_salad);

    println!("Remove last fruit from salad.");
    fruit_salad.pop();
    print_statistic(&fruit_salad);

    println!("Remove 'apple' from salad.");
    remove_fruit(&mut fruit_salad, "apple");
    print_statistic(&fruit_salad);
}

fn remove_fruit(salad: &mut Vec<&str>, fruit: &str) {
    let index = salad.iter().enumerate().find(|(_, f)| ***f == *fruit);
    if let Some((i, _)) = index {
        salad.splice(i..(i + 1), vec![]);
    }
}

fn print_statistic(salad: &Vec<&str>) {
    let mut fruits = HashMap::new();
    salad
        .iter()
        .for_each(|fruit| *fruits.entry(fruit).or_insert(0) += 1);

    let mut ingredients: Vec<_> = fruits
        .iter()
        .map(|(fruit, count)| format!("{}: {}", fruit, count))
        .collect();
    ingredients.sort();

    let ingredients = ingredients.join(", ");
    println!("Fruit salad: [{}]", ingredients)
}
