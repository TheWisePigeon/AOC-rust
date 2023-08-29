use rand::prelude::*;
use std::io;

fn main() {
  let mut rng = thread_rng();
  let number_to_be_guessed = rng.gen_range(1..100);
  let mut number_guessed = false;
  while !number_guessed {
    let mut user_guess = String::from(""); 
    println!("Guess what...");
    io::stdin().read_line(&mut user_guess).expect("Error while reading input");
    match str::parse::<i32>(&user_guess.trim()){
      Ok(guess)=>{
        if guess == number_to_be_guessed {
          println!("Hooray you got it");
          number_guessed = true
        }
        if guess > number_to_be_guessed {
          println!("Lower");
        }
        if guess < number_to_be_guessed {
          println!("Higher");
        }
      },
      Err(err)=>{
        println!("{:?}", err);
      }
    }
  }
}
