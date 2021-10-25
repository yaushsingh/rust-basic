use core::num;

fn main() {
    /* if statements*/
    let number = 40;

    if number < 10{
        let sum = number +10;
        println!("the new number is {}",sum);
    }
else if number > 17 {
    println!(" greater that 17")
}

else{
    println!("third condtion satisfied")
}

    // if and else statement can be used in let statement
    let condition = true;
    let number = if condition {4} else {6};
    println!("the number is {}", number);


    /* creating loops in rust */
    let mut counter = 0;
     /* we can use result of loops to assign the variablees */
    let result = loop{
        counter +=1;
        println!("infinite loop {}", counter);

        if counter == 5{
            break counter;
        }
    };
    println!("the result is {}", result);


    /* while loop */
    let mut x = 3;
    while x != 0{
        println!("the value of x is {}", x);
        x -=1;
    }

    /* for loop
    it is used for looping in collection */
    /* array arecollection of same data type */
    let a = [10, 2, 44, 3, 0, 33];

    for element in a.iter(){
        println!("the element in a is {}", element);
    }

    // using for loop with range
    for ele in (1..6){
        if ele%2 == 0{
            println!("{} is even", ele);
        }
    }

     

}
