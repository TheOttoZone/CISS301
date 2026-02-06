use std::io;
use std::collections::HashSet;
use std::fs::File;
use std::iter;
use std::slice;
use std::str;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_name = File::open(args[1]);
    let mut contents:String = String::new();
    let mut use_file:bool = false;

    // if an argument was made, attempt to open the argument as a file
    if args.len() > 1 {
        
        file_name = File::open(args[1].clone());
        file_name.read_to_string(&mut contents);
        use_file = true;
    }
    

}
