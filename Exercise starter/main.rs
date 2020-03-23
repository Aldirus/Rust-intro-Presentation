// un comment these as you need them.
// use std::io;
// use std::io::stdout;
// use std::io::BufWriter;
// use std::io::Write;
// Add using for ferris-says

fn main() {


    let mut first_name = //Finish me

    let mut last_name = //Finish me

    let mut max_num = //Finish me


    //Resulting number from fibonacci challenge
    let mut even_sum = //Finish me


    //This part will convert your values into a byte slice for ferris-says
    let stdout = stdout();
    let message = format!(
        "{} {}{} {}",
        &first_name,
        &last_name,
        ", your number is :",
        &even_sum.to_string()
    );

    //use these three variables for your ferris-says
    let out = message.as_bytes();
    let width = out.len();
    let mut writer = BufWriter::new(stdout.lock());

}
