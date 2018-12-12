use std::env;
use std::fs;

fn main() {
    // simple file reader from command line
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1]; 
    let f_name = &args[2];

    if query == "read" {
        println!("reading from file {}:", f_name);
        let content = fs::read_to_string(f_name)
            .expect("Cannot read file");
        
        println!("{}", content);
        
    }
}