use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);
    
    println!("Guess the number!");

    loop {
      println!("Please input your guess.");

      //vars immutable by def need mut tag
      let mut guess = String::new();

      io::stdin()
          //references also immutable by default
          .read_line(&mut guess)//returns ok or err
          .expect("Failed to read line");

      println!("You guessed: {}", guess);

      //CONTROL FLOW WITH ERROR HANDLING USING MATCH INSTEAD OF EXPECT
      //EXPECT THROWS THE ERROR
      let guess: i32 = match guess.trim().parse() {
            Ok(num) => num ,
            Err(_) => continue,
      };

      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
            println!("You win!");
            break;
          },
      }
    }
    
}