// similar to the using statement in C++
use std::io;

fn main() {
    println!("Guess the number between 1 and 100!");
    println!("Please input your guess: ");

    // creating a number to guess
    let num = 21;
    // creating a mutable string to store user's guess
    let mut guess = String::new();

    // reading the user's input and passing it to the guess var. by reference
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: i32 = guess.trim().parse()
        .expect("Type a real number please!");

    println!("Your guess was: {}", guess);
    if  num == guess {
        println!("You guessed correctly!");
    }
    else {
        println!("You did not guess correctly! Try again.")
    }
}
