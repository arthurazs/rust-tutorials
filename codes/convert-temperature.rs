use std::io::stdin;

fn convert(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn main() {
    // Convert temperatures between Fahrenheit and Celsius
    loop {
        println!("Enter the temperature in Fahrenheit:");

        let mut fahrenheit = String::new();
        stdin().read_line(&mut fahrenheit)
            .expect("Failed to read fahrenheit");

        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Fahrenheit should be a number, try again");
                continue
            }
        };

        let celsius: f32 = convert(fahrenheit);

        println!("\n{}°F = {}°C", fahrenheit, celsius);
        break;
    }
}
