// we want the args function from env but its good practice to bring out the parent module instead of just the function
use std::env;
use std::fs; // need this to handle files 
use std::io::{self, BufRead};
fn main() {

    let args: Vec<String> = env::args().collect(); // args() creates a iterator, .collect() makes it a collection 
    let stdin = io::stdin();

    let mut content: String = String::new();  
    
    if args.len() == 2 {        
        content = fs::read_to_string(&args[1]) // filename
        .expect("Something went wrong when reading file");
    }
    else{
        for line in stdin.lock().lines(){
            let line = line.expect("Could not read line from standard in");
            content.push_str(&line);
            content.push_str("\n");
        } 
    }
   
    println!("\n{}",content);

    
        // println!("{:?}", args); // println! is a macro not a function, so it rewrites the code on runtime
        // println!("\n{}",content);
    
}
// If you run into the linker cc issue its because you don't have gcc installed 
// To read the file you need to do cargo run in the same location as the Cargo.toml file

