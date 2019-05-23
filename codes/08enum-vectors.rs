fn main() {
    enum Cell {
        Int(i32),
        Float(f32),
        Text(String)
    }

    let vector: Vec<Cell> = vec![
        Cell::Int(3), Cell::Float(3.3),
        Cell::Text(String::from("three"))];

    for value in &vector {
        match value {
            Cell::Int(number) => println!("i32: {}", number),
            Cell::Float(number) => println!("f32: {}", number),
            Cell::Text(number) => println!("String: \"{}\"", number)
        }
    }
}
