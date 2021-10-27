/*    ----- Ownership rules-----
1. Each Value in Rust has a variable that's called its owner
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/
fn main() {

    let x = 5;
    let _y = x; // Copies the value 5 into y
    // it is convention in rust to use underscore "_" to represent unused variable
    let s1 = String:: from("hello");
    let s2 = s1; // moved
    // In Rust the s1 and s2 does not share the pointer for the value
    // instead here the value of s1 is moved 
    let _s3 = s2.clone();
    
    println!("{}, world", s2);// we cloned the value of s2 so we can use the s2 variable
    // but using println!("{} world", s1); gives error 
    // as the value of s1 is moved to s2
    abt_ownership();
    abt_referencing();
}


// now lets see example of ownership 
fn abt_ownership(){
    let s = String::from("hello");
    takes_ownership(s);
    /* passing the parameter into function is equivalent to 
    assigning the parameter to another variable 
    i.e. value is moved so below line gives error */
    //println!("{}",s);

    let x = "ss";
    makes_copy(x);
    //since the value of int, bool, char and string literal are copied
    //we dont get error
    println!("{}",x);

    let a_string = gives_ownership();
    println!(" I am learning {}",a_string);


    // we can also use function for taking and giving the ownership
    let string1= gives_ownership(); 
    let string2 = String:: from ("hobbyist");
    let string3 = takes_and_gives(string2);
    println!("I code in {} as {}", string1, string3 );
}

    fn takes_ownership(some_string: String){
        println!("{}", some_string);
    }

    fn gives_ownership() -> String {
        let some_string = String:: from ("Rust");
        some_string
        //returning string moves the ownership to the a_string variable of this example
    }
 //in this example below function takes ownership from string2 and gives it to string3
    fn takes_and_gives(any_string: String) -> String{
        any_string
    }

    fn makes_copy(some_str: &str ){
    println!("{}",some_str);
    }


// taking and giving ownership is tedious work
// so lets learn about referencing in Rust
fn abt_referencing(){
    let first_str = String :: from ("ayush");
    let  len = calculate_length(&first_str); //using "&" we pass the reference of First_str into the function
    println!("the length of {} is {}", first_str, len);

    refence();
}

    fn calculate_length(str: &String)-> usize{ // str takes the reference of string and the reference are immutable
        let length = str.len(); // .len() is built in funtion that returns the length ofstring
        length // these funtion returned a tuple of string and number
    }

    //mutable and immutable refrences
    fn refence(){
         let mut s = String:: from("apple");
 
         let r1 = &s;
         let r2 = &s; // we cannot have two mutable reference for same data in particular scope dto avoid data races
         // in the line below error will occur as immutable reference alreadyexist
         //let r3 = &mut s;

         println!("{} {} ", r1,r2);
          // out of the scope of r1 and r2
          // we can write
          let r3 =&mut s;
          println!("{} is fruit",r3);
    }

/* 
 Rules of References
 - at any given time, you can have either one mutable reference
    or any number of immutable references
 - reference must always be valid
*/