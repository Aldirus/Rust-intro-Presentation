# Rust Notes  

- [Installation](#installation)
- [cargo](#cargo)
  * [Create new project](#create-new-project)
  * [Building a project](#building-a-project)
- [Intro stuff](#intro-stuff)
  * [var declaration](#var-declaration)
  * [Using](#using)
  * [:: symbol](#---symbol)
- [Simple Collections](#simple-collections)
  * [Tuple](#tuple)
  * [Array](#array)
- [Control Flow](#control-flow)
  * [if expression](#if-expression)
  * [Loops](#loops)
- [loop](#loop)
- [while](#while)
- [for](#for)
  * [OwnerShip](#ownership)
    + [Rules of ownership](#rules-of-ownership)

## Installation
Installation (Windows)  
Install rustup (command line tool for managing Rust versions and associated tools)  
https://www.rust-lang.org/tools/install

install Build Tools for Visual Studio 2019 under Tools for Visual Studio 2019  
https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019

To check whether you have Rust installed correctly use this command.  
`$ rustc --version`

Rust discord
https://discordapp.com/invite/rust-lang

Rust local documentation  
`$ rustup doc`

## cargo

### Create new project  
To create a new project with cargo functionality  
`$ cargo new hello_cargo`  
This will auto generate our files for us including Cargo.toml.  
Cargo.toml holds our package information and our dependencies. One of rust strong suits is it's library usage (crates).  

### Building a project  
Debug compile:  
`$ cargo build`  
Will create an executable on your computer which you can run at target/debug/project_name.exe

Release compile:  
`$ cargo build --release`  
This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible.

Check if compilable:  
`$ cargo check`  
cargo check is much faster than cargo build, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using cargo check will speed up the process.

Run code using cargo:  
`$ cargo build`  
Easy way to compile and run your code together.

## Intro stuff

### var declaration
 In Rust, variables are immutable by default. 
 ex:  
 ```rust
 let foo = 5; // immutable
 let mut bar = 5; // mutable
 ```
 Trying to override a non-mutable variable will throw a compile time error. Even though compiler errors can be frustrating, they only mean your program isn’t safely doing what you want it to do yet.

>  [What's the difference between constants and immutable variables in rust?](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants "Differences Between Variables and Constants")

### Using
To use methods from the standard library you need to add them to the prelude.  
add `use std::io;` to the top of your document to get access to in out methods such as `io::stdin().readline()` which is equivalent to  `Console.ReadLine();` in c#.

To use an external crate you need to specify the crate in the Cargo.toml under `[dependencies]` for example:  
```toml
[dependencies]
rand = "0.5.5"
```  
 Cargo fetches the latest versions of everything from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

 Run `$cargo build` to add the dependenices and their core dependices to your project.


### :: symbol
The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.  
ex: 
```rust
let mut guess = String::new();
```

## Simple Collections

### Tuple

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

ex: Declaring tuple with annotations
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

ex: Declaring, Destructuring, Printing
```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```
This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring, because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.


### Array
Every element of an array must have the same type. arrays in Rust have a fixed length, like tuples.

ex: 
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    
}
```

```rust
  let a: [i32; 5] = [1, 2, 3, 4, 5];
```
Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

Let's say we run this command after declaring a from above:   
` let element = a[10];`

The program results in a runtime error and wont exit successfully. When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the array length, Rust will panic.

In many low-level languages, this kind of check is not done.

## Control Flow

### if expression

The if expression behaves most like any other language. Here's an example to get an idea of the syntax. 

ex:
```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
Each path of the if expression is called arms.


In Rust `if` is an expression rather than a statement meaning that we can use it on the right side of a `let` statement.

ex: 
```rust
let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```
Note: The result of each arm must be of the same type in this case.

> [Statement vs Expression](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions "Go to documentation")

### Loops

## loop

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

ex:
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

Another example of assigning a value using an expression.

## while

While loop runs until a condition is met. Works just like any other language you are familiar with.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

## for

ex: Looping through an array

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

ex: Looping through a range 
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

### OwnerShip
Every language has to deal with how it manages it's memory.
We are familiar with garbage collection from C# and Java. Many other languages the programmer must explicitly allocate and free memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

#### Rules of ownership

  * Each value in Rust has a variable that’s called its owner.
  * There can only be one owner at a time.
  * When the owner goes out of scope, the value will be dropped.




