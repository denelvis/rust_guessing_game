use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret nuber is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess).value();
        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
