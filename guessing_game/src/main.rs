
use std::io;
use std::cmp::Ordering;
use rand::Rng;
//bringing color library to scope
use colored::*;


fn main(){
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is {}", secret_number);
    
    //using loop funcion to make game continuous
    loop{
    println!("Please input your guess:");


    //guess is variable of St ring type
    //by default guess variable is immutable so we use mut 
    let mut guess: String = String::new();

    //taking the infromation from user using io librabry
    io::stdin()
        .read_line( &mut guess).expect("Failed to read line");

    //changing the string input from user into integer
    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_)=> continue,
    };
    
    println!("You guessed :{}", guess);      

    match guess.cmp(&secret_number){
        Ordering::Less => println!("{}","Too small".red()),
        Ordering::Greater => println!("{}", "Too big".red()),
        Ordering::Equal => {println!("{}", "You won".green());
                                    break;
                                },
    }
    }
}

