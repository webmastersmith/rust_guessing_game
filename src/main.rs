use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    println!("Guess Number between 1 and 5.");
    let secret_number = rand::thread_rng().gen_range(1..=5);
    // println!("SSHHHHHH! Secret number is: {}", secret_number);
    
    // get guess from user and print it.
    loop {
        println!("Please input your guess.");
        //create place to store info in buffer.
        //String::new, a function that returns a new instance of a String
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess); 
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
