use std::io::stdin;

fn main() {
    println!("What is 3+2? Type your answer and press enter.");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input");

    //if input == "5" { 

    if input.trim() == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");        
    }

    // debug output "5\n"
    println!("{:#?}", input);
}
/* 
if input == "5"... without trim
What is 3+2? Type your answer and press enter.
5
Incorrect!
*/
