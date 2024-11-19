use std::cmp::Ordering;
use std::io;
use rand::Rng;
//input output library

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("Not so secret number is {secret_number}");

    loop{
        let mut guess = String::new(); //variable mutable

        io::stdin()
            .read_line(&mut guess) // read_line get user input || & donc une référence?
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
