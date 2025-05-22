use std::io::{self, Write};

fn main() {
    // Show the menu
    println!("This program can give you nth Fibonacci number at the given index");

    loop {
        println!("Which index do you want to find?");
        print!("> ");
        // flush so the prompt appears before read_line blocks
        io::stdout().flush().unwrap();

        let mut temp = String::new();
        // flush so the prompt appears before read_line blocks

        // Read input
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read lines");

        let temp: u128 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let result = loop_fibonacci(temp);
        println!("The Fibonacci number at index {} is {}", temp, result);
    }

    fn rec_fibonacci(n: u128) -> u128 {
        if n <= 1 {
            return n;
        }
        rec_fibonacci(n - 1) + rec_fibonacci(n - 2)
    }

    fn loop_fibonacci(n: u128) -> u128 {
        if n <= 1 {
            return n;
        }
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}
