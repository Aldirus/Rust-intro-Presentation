pub fn function_name(){
    println!("We put the fun in functions!");
}

pub fn string_formatting(){
    //basic string formatting
    println!("The weather is going to be {} this weekend.", "bad");
    //string formatting using positional arguments
    println!("{0} is a hard worker, but unfortunately he {1}. Don't be like {0}.","Jeremy","has a man bun");
    //string formatting using named arguments
    println!("{teacher_name}, the grade we would like to recieve on our project about {subject} is {grade}. Thanks!",teacher_name="Stephen",grade = 100,subject = "Rust");
}

pub fn learning_variables(){
    //to declare a variable use let
    let favorite_meal = "Chicken and Veggies";
    println!("Jeremy's favorite meal is {}", favorite_meal);

    //to declare a mutable variable add mut along side let
    let mut best_dev = "Jeremy";
    println!("{} is the best developer in WMAD.",best_dev);
    best_dev = "Anthony";
    println!("Just kidding, {} is the best developer in WMAD!",best_dev);

    //to declare a static variable use static
    static number: i32 = 100;
}

pub fn cool_crates(){
    let language = "Rust";
    println_f!("We love coding {language}!");
}

pub fn loops(){
    //Loops from 0 - 9
    println!("For Loop:");
    for x in 0..10 {
        println!("{}", x);
    }
    //while y number isnt 128 add its value to its self
    println!("While Loop:");
    let mut y = 1;
    while y != 128 {
    y += y;

    println!("{}", y);
    }
}

pub fn magic_match(){
    let name = "Jeremy";

    match name {
    "Stephen" => println!("{}, What he's a great teacher, very funny and extremely smart",name),
    "Anthony" => println!("{}, he's really cool guy.",name),
    "Jeremy" => println!("{}, he's alright I guess",name),
    _ => println!("someone else"),
    }
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };
}