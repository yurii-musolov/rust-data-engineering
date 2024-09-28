use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    print_fruits(&fruit);
}

fn print_fruits(fruits: &LinkedList<&str>) {
    println!(
        "Fruit list : {}",
        fruits.clone().into_iter().collect::<Vec<_>>().join(", ")
    );
}
