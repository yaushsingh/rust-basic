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

    abt_datatypes();

  let sum = addition(4,27);
    println!("the sum is {}", sum);
}

//preparing function 
fn abt_datatypes(){
    /* rust has scalar and compund datatypes
    scalar types : integer, floating-point, boolean, characters
    
    */
    let a = 2;
    let b: u32 = 4;
    let sum = a+b;
    println!("the sum of {} and {} is {}",a,b,sum);
    let c = 5.66;
    let d = 4.5;
    let diff = c-d;
    println!("the difference of {} and {} is {}", c,d,diff);

    let t = true;
    let x = false;
    println!("this is {} as well as {}",t,x );

    //compound datatypes
    let tup = ("ayush", 34);
    //tuple can be accessed by destructuring and dot notation 

    // destructuring 
    let (name, age) = tup;
    //dot notation
    let age = tup.1;
    println!("my name is {} and i am {} years old.", tup.0, tup.1);


    let error_codes= [401,402,404,405];
     let not_found = error_codes[2];
     println!("error code {}",not_found);
} 

//simple add function that returns sum
fn addition(x: i32 ,y:i32)-> i32{
    /* return x+y can be written as below because in rust by default
     last line of function is retrun value*/
    x +y
}
    