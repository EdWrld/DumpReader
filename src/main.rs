// we want the args function from env but its good practice to bring out the parent module instead of just the function
use std; 
use std::env;
use std::fs; // need this to handle files 
use std::io::{self, BufRead};

fn main() {

    let args: Vec<String> = env::args().collect(); // args() creates a iterator, .collect() makes it a collection 
    let stdin = io::stdin();
    let mut disk_dump: String = String::new(); 
    // println!("{:?}", args);

    if args.len() >= 2 && &args[1][0..2] != "-" { // user inputted a file as a parameter
         disk_dump = fs::read_to_string(&args[1]) // read file
        .expect("Something went wrong when reading file");
    }
    else { // user piped input 
        for line in stdin.lock().lines(){      
            let line = line.expect("Could not read line from standard input");
            disk_dump.push_str(&line);
            disk_dump.push_str("\n"); 
            
        }
    }

    output_dump(disk_dump);
    extra_arg_functionality(args);
   
}

fn extra_arg_functionality(args: Vec<String>){
    for arg in args {
        if arg == "-v" || arg == "-V"{
            println!("Version 1.0");
        }
        else if arg == "-h" || arg == "-H"{
            println!("\nInput a your dumpfile or pipe it when before cargo run \nExample: cargo run filename.txt or filename.txt | cargo run \n-v - version # \n-h - help");
        }
    }
}

// fn insert_type_with_gap(dump: String ,typ: String, line: &String){
//   dumpl .push_str(&line[0..2]);
//   dumpl .push_str(typ);
//   dumpl .push_str("               ");    
//   dumpl .push_str(&line[4..]);
//   dumpl .push_str("\n")
// }
// decodes the ascii areas of the dump and outputs it

fn output_dump(dump:String) {

    let mut index : i32 = 0;
    let mut decoded_dump: String = String::new();
    let mut is_root: bool = true;
    
    for line in dump.lines(){

        if index ==0{
            decoded_dump.push_str(&line[0..3]);
            decoded_dump.push_str("                ");
            decoded_dump.push_str(&line[3..]);
            decoded_dump.push_str("\n");
        }else if index ==1 {
            decoded_dump.push_str(&line[0..3]);
            decoded_dump.push_str(" ");
            decoded_dump.push_str(&line[3..4]);
            decoded_dump.push_str("               ");    
            decoded_dump.push_str(&line[4..]);
            decoded_dump.push_str("\n")

        }else if index>= 2 && !is_root{ // the split is exclusive not inclusive

            match &line[3..4]{
                "0" => 
                 {
                    decoded_dump.push_str(&line[0..3]); 
                    decoded_dump.push_str(" folder");
                    decoded_dump.push_str("               ");
                    decoded_dump.push_str(&line[4..]);
                    decoded_dump.push_str("\n")
                    },
                "1" =>
                 {
                    decoded_dump.push_str(&line[0..3]);  
                    decoded_dump.push_str(" empty cluster");
                    decoded_dump.push_str("   ");
                    decoded_dump.push_str(&line[4..]);
                    decoded_dump.push_str("\n");
                },
                "2" => {
                    decoded_dump.push_str(&line[0..3]);  
                    decoded_dump.push_str(" damaged cluster");
                    decoded_dump.push_str("               ");
                    decoded_dump.push_str(&line[4..]);
                    decoded_dump.push_str("\n");

                },
                "3" => {
                    decoded_dump.push_str(&line[0..3]); 
                    decoded_dump.push_str(" file header");
                    decoded_dump.push_str("     ");
                    decoded_dump.push_str(&line[4..7]);
                    decoded_dump.push_str(&line[8..].to_ascii_lowercase());
                    decoded_dump.push_str("\n");

                },
                "4" => {
                    decoded_dump.push_str(&line[0..3]); 
                    decoded_dump.push_str(" file data");
                    decoded_dump.push_str("       ");
                    decoded_dump.push_str(&line[4..5]); 
                    decoded_dump.push_str(&line[6..].to_ascii_lowercase());
                    decoded_dump.push_str("\n");


                }
                _ => println!("The Cluster type is unknown")
            }
    
        }
    else{
        decoded_dump.push_str(&line[0..3]);
        decoded_dump.push_str(" root cluster"); 
        decoded_dump.push_str("    ");
        decoded_dump.push_str(&line[4..8]);
        decoded_dump.push_str(&line[9..].to_ascii_lowercase());
        decoded_dump.push_str("\n");
        is_root = false; 
        }
    index+=1;
    } 
println!("{}",decoded_dump);

}


// If you run into the linker cc issue its because you don't have gcc installed 
// To read the file you need to do cargo run in the same location as the Cargo.toml file
// println! is a macro not a function, so it rewrites the code at runtime
// To add new imports you have to add the imports to Cargo.toml 
