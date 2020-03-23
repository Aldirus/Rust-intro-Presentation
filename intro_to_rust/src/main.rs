#[macro_use]
extern crate fstrings;
extern crate ferris_says;

use ferris_says::say;

mod basics;

//input output
use std::io;
use std::io::stdout;
use std::io::BufWriter;

fn main() {
    basics::function_name();
    basics::string_formatting();
    basics::learning_variables();
    basics::cool_crates();
    basics::loops();
    basics::magic_match();
    // let name = input("What is your name?").expect("An Error Occurred");
    // let age = input("Enter your age:").expect("Failed to get age.").parse::<i32>().expect("Invalid age.");
    // println!("Hello, {}! You are {} years old.",name,age);
}

//Displays message for user and prompts them to a give a response
//we may return an io result error so io::Result before return type signifies that it can fail and that it can handle it 
fn input(message: &str) -> io::Result<String>{
    //Allows you to write data into an object
    use std::io::Write;

    print!("{}",message);
    //stdout or standard out is what you display to the user
    //question mark tells rust to return early on failure
    //flush ensures that all currently buffered content gets pushed out to the user
    //we flush here because we want our message to display right away
    io::stdout().flush()?;
    
    //set the buffer as an empty growable string
    let mut buffer: String = String :: new();
    //stdin or standard input is the users input allows user to write until the user hits enter
    io::stdin().read_line(&mut buffer)?;

    //trim end gets rid of the new line
    //because this makes reference to the string we have to make it owned 
    Ok(buffer.trim_end().to_owned())
}
