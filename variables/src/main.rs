// in this program we will study about the variales 
/*fn main() {
    // variables are immutable by default in rust
    let x = 5;
    println!("The value of x is {}", x);
    x = 6;
    // here assigning x = 6 gives error
}
*/
fn main() {
    // variables are immutable by default in rust
    // we use mut keyword to make variable mutable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;

    println!("now the value of x is {}", x );

    //lets exprore the concept of shadowing
    let  x = 30;
    println!("The value of x is {}", x);
    let x : &str = "31";
    //by shadowing we can preserve mutability and change types for variable
    println!("now the value of x is {}", x );

    // rust uses const keyword for concept of constants
    const MY_YEAR : u32 = 34;

}