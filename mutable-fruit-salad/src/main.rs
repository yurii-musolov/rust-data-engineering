fn main() {
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad: {:?}", fruit_salad);

    fruit_salad.push("figs");

    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    fruit_salad.push("figs");
    println!("Modified fruit salad: {:?}", fruit_salad);
}
