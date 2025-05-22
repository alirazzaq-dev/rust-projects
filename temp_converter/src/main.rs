use std::io::{self, Write};

fn main() {
    // 1. Ask user what conversion they want to perform like 1. f to c 2. c to f
    // 2. Takes input from user and should be a number
    // 3. Show the result
    // 4. Quit Automatically

    // Show the menu
    println!("Choose an option:");
    println!("1. Convert Celsius to Fahrenheit");
    println!("2. Convert Fahrenheit to Celsius");

    loop {
        print!("> ");
        // flush so the prompt appears before read_line blocks
        io::stdout().flush().unwrap();
        // // Read input
        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read input");
            return;
        }

        match choice.trim() {
            "1" => {
                handle_c_to_f();
                break;
            }
            "2" => {
                handle_f_to_c();
                break;
            }
            _ => {
                println!("please enter 1 or 2 next time");
                continue;
            }
        }
    }

    fn handle_c_to_f() {
        println!("Enter temperature in Celsius");

        loop {
            print!("> ");

            // flush so the prompt appears before read_line blocks
            io::stdout().flush().unwrap();

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read lines");

            let temp: u32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // logic to convert celsius to fahrenheit
            // F = C * 9/5 + 32
            let fahrenheit = (temp as f32 * 9.0 / 5.0) + 32.0;
            println!("Temperature in Fahrenheit: {:.2} F", fahrenheit);
            break;
        }
    }

    fn handle_f_to_c() {
        println!("Enter temperature in Fahrenheit");

        loop {
            print!("> ");

            // flush so the prompt appears before read_line blocks
            io::stdout().flush().unwrap();

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read lines");

            let temp: u32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // logic to convert fahrenheit celsius
            // C = (F - 32 ) * 5 / 9
            let celsius = (temp as f32 - 32.0) * 5.0 / 9.0;
            println!("Temperature in Celsius: {:.2} C", celsius);
            break;
        }
    }
}
