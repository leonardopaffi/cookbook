use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println! Rust macro
    println!("Guess the number!");
    // creating random number (open documnetation using cargo doc --open)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // starting loop
    loop {
        println!("Please input your guess");
        //creating and empty mutable variable of type string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)                      // ref value must be mutable too
            .expect("Failed to read the line");         // if it returns an error, print this line and crash program

        println!("You guessed: {}", guess);             // {} is for printing variables listed after the main string

        // Shadowing previous variable with a new value and type
        // In Rust there's no need to declare a new variable like i_guess for different types
        // trim()   ->  removes initial whitespace and final \n
        // parse()  ->  converts a string to a number
        let guess: u32 = match guess.trim().parse() {   // checking if it's a number or not
            Ok(num) => num,                             // if it's a number, return that number
            Err(_) => continue,                         // if it's NOT a number, restart loop
        };

        match guess.cmp(&secret_number) {               // checking guess variable using Ordering
            Ordering::Less => println!("Too small!"),   // guess < secret_number
            Ordering::Greater => println!("Too big!"),  // guess > secret_number
            Ordering::Equal => {                        // guess == secret_number
                println!("You win!");
                break;                                  // ending the program
            },
        };
    }
}
