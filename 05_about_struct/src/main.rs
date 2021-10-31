/*
    A struct, or structure, is a custom data type that lets you name and
    package together multiple related values that make up a meaningful 
    group. If you're familiar with an object-oriented language, a struct
    is like an object's data attributes.
    Structs are similar to tuples. Like tuples, the pieces of a struck can
    be different types. Unlike with tuples, you'll name each piece of data
    so it's clear what the values mean. As a result of these names, structs
    are more flexible than tuples: you don't have to rely on the order of
    the data to specify or access the values of an instance.
*/



//creating User struct with many data in rust using struct keyword
struct User{
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool,
}
#[derive(Debug, Copy, Clone)]
struct cubiod{
    width: u32,
    length : u32,
    height: u32,
}
// implementation block is used for the method and function of our struct
impl cubiod{
    fn volume(&self) -> u32{   //using the reference
        self.height* self.width *self.length
    }

    fn can_hold(&self, other :cubiod) -> bool {
        self.volume() > other.volume()
         
    }
}

//in rust we can use multiple implementaion block for struct
impl cubiod{
    fn cube(size :u32 ) -> cubiod{
        cubiod { width:size, length: size, height: size }
    }
}

// method and associated function can be differentiated as
// method passes self where as associated function doesnot 

/* in case of tuple we access the data by using index
while using struct we can acess the data using variable name  */
fn main() {
    //now lets create new instance variable
    let mut user1 = User{
        email: String::from("abc@abc.com"),
        username: String:: from("hela123"),
        active: true,
        sign_in_count : 1
    };
// we can use dot notation to get specific values from our struct
    let name = user1.username;
    //to change the value of the user1 struct we need it to be mutable
    user1.username = String::from("thor123");

    let user2 = build_user(String::from("asd@asd.com"), String::from("odin123"));

    // we can new instances of struct using existing instances
    let user3 = User {
        email: String::from("qwerty@as.com"),
        username: String:: from("loki123"),
        .. user2 //here the user3 will get the email and username of above but other value of user2
    };

    //we can also create a tuple struct
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    //tuple struct are useful when we want to name our entire tuple and separate them for 
    //each other and function expects separate  tupple Point and Color



    //now using these knowledge lets find the area of rectangle
    let cube = cubiod{height:22, width : 33, length:50}; //creating a tuple
    //println!("The volume of cubiod is {} pixels", volume(&cube));
    // after using the implementation block we can use
    println!("The volume of cubiod is {} pixels", cube.volume());

    //now lets print our cube instance
    println!("cube: {:?}", cube); //so simple data typesuch as int have display trait but struct doesnot have the builtin display traits 
    //so we use {:?} instead of {}
    // also struct does not have debug trait so we add #[derive(Debug)] while creatingstruct

    let cuboid1 = cubiod{height:23, width:11, length:32};
    let cuboid2 = cubiod{height:11, width:22, length : 5};
    let cuboid3 = cubiod::cube(34);

    println!("cubiod 2 can hold: {} volume", cuboid2.volume());
    println!("cubiod 1 can hold: {} volume", cuboid1.volume());

    println!("cubiod 1  can hold cuboid 2 : {}", cuboid1.can_hold(cuboid2));
    println!("cubiod 2 can hold cuboid 1 : {}", cuboid2.can_hold(cuboid1));
}

/* // we are passing the reference of cubiod to use its field but not take the ownership
fn volume(cube: &cubiod) -> u32{ 
    cube.height* cube.width *cube.length
}

since this function is related to rectangle we can create the method of struct cubiod using impl 
to compact the program code
*/



// we can use function to construct new instances of user
fn build_user(email:String, username:String) -> User{ // -> User indicates that we are returning a User type from this function
    User{
        email: email,  //email of this struct should the same passed by the new user
        username, //we can simplify the declaration by using field init shorthand syntax
        active: true,
        sign_in_count:1,
    }
}
