use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input your guess:")

    let mut guess: String = String::new();

    io::stdin(): stdin
        .read_line(buf: &mut guess): Result<usize,Error>
        expect(msg:"Failed to read line");

    println!("You guessed :{}", guess);      
}
