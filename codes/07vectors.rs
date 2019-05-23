
fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);

    println!("The third element is {}", &vector[2]);

    for value in &mut vector {
        *value += 3;
    }

    match vector.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

}
