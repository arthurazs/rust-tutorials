use std::io::stdin;
use std::process::exit;

fn main() {
    // Generate the nth Fibonacci number
    let mut fibonacci: (u128, u128) = (0, 1);

    let mut nth = String::new();
    println!("Pick a number between 1 and 186:");
    stdin().read_line(&mut nth)
        .expect("Failed to read the nth number.");

    let nth: u8 = match nth.trim().parse() {
        Ok(number) => {
            if number <= 0 {
                println!("ERROR The number should be higher than 0");
                exit(-1)
            }
            else if number > 186 {
                println!("ERROR Can't calculate over the 186th fibonacci number");
                exit(-1)
            }
            number
        },
        Err(_) => {
            println!("ERROR Please enter a number between 1 and 186");
            exit(-1)
        }
    };

    for _ in 1..nth {
        fibonacci = (fibonacci.1, fibonacci.0 + fibonacci.1)
    }

    println!("The {}th Fibonacci number is {}", nth, fibonacci.1);
}
