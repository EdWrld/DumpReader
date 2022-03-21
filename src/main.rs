// we want the args function from env but its good practice to bring out the parent module instead of just the function
use std; 
use std::env;
use std::fs; // need this to handle files 
use std::io::{self, BufRead};

fn main() {

    let args: Vec<String> = env::args().collect(); // args() creates a iterator, .collect() makes it a collection 
    let stdin = io::stdin();

    // println!("{:?}", args);

    let mut content: String = String::new(); 


    if args.len() >= 2 && &args[1][0..2] != "-" { // user inputted a file as a parameter

        
        let temp_content: String = fs::read_to_string(&args[1]) // read file
        .expect("Something went wrong when reading file");

        // removing lines that arn't needed. 
        let mut index : i32 = 0;
            for line in temp_content.lines(){
                if !(index <2){
                    content.push_str(line);
                    content.push_str("\n");
                }
                index +=1;        
            }
        println!("{}", content);
    }
    else { // user piped input 
        for line in stdin.lock().lines(){
            
            let line = line.expect("Could not read line from standard input");
            content.push_str(&line);
            content.push_str("\n"); 
            
        }
    }
   
   extra_arg_functionality(args);
   // println!("{}",content);
   
   
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

enum ClusterType {
    Root, // Root cluster, decode everything past column 07 
    Folder, // 
    Empty, // 
    Damaged, // 
    Fileheader, // everything past column 05 is what we need to decode
    Filedata, // everything past column 03 is what we need to decode  
}

struct cluster{
    kind: ClusterType
} 

// decodes the ascii areas of the dump and outputs it 
fn output_dump(dump:String){

    let mut decoded_dump : String = String::new();

    let mut is_root: bool = true;

    if is_root != true{
        
    for line in dump.lines(){

        match &line[3..3]{
           "0" => 
           {
               decoded_dump.push_str("folder");

            },
            "1" =>
             {
               decoded_dump.push_str("empty cluster");

            },
            "2" => {
               decoded_dump.push_str("damaged cluster");

            },
            "3" => {
               decoded_dump.push_str("file header");

            },
            "4" => {
                decoded_dump.push_str("file data")
            }
            _ => println!("The Cluster type is unknown")
        }
    }
}else{
    //read cluster 
    is_root = false;
    }


}


// If you run into the linker cc issue its because you don't have gcc installed 
// To read the file you need to do cargo run in the same location as the Cargo.toml file
// println! is a macro not a function, so it rewrites the code at runtime
// To add new imports you have to add the imports to Cargo.toml 
