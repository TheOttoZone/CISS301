use std::hash::Hash;
use std::io;
use std::collections::HashSet;
use std::fs;
use std::str;
use std::env;

fn main() -> std::io::Result<()> {
    // collect command line args
    let args: Vec<String> = env::args().collect();
    let mut use_file:bool = false;

    // if an argument was made, attempt to open the argument as a file
    if args.len() > 1 {
        let file_name = args[1].clone();
        let raw_text = fs::read_to_string(file_name).expect("Could not read file.");
        let content: Vec<&str> = raw_text.split('\n').collect(); // split up the file content by line and store it to be called by line number
        use_file = true;
        
        match content[0].trim().to_lowercase().as_str() { // the first line of the file should be the spec of data, so act depending on that

            "integer" => {
                let a: HashSet<i32> = content[1]
                    .split_whitespace() //split the content up into pieces between spaces
                    .map(|s| s.parse::<i32>()) // convert &str to int
                    .collect::<Result<HashSet<i32>, _>>() 
                    .expect("Found non integer values!");

                let b:HashSet<i32> = content[2]
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<HashSet<i32>,_>>()
                    .expect("Found non integer values!");
                
                print_results(a, b);  
            }

            "string" => {
                let a:HashSet<String> = content[1].split_whitespace().map(String::from).collect(); // String::from converts &str to string
                let b:HashSet<String> = content[2].split_whitespace().map(String::from).collect();

                print_results(a, b);
            }

            _ => {
                println!("First line of file is not either \"integer\" or \"string\".");
            }
        }

    }

    if use_file == false {

        println!("Since you did not supply a file, select a data type to input:\n1. String\n2. Integer\n-----------");

        loop{
            let input = get_int_input();

            match input{
                1 => {
                    loop{
                        println!("\nType space separated words to insert into set A:");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let a:HashSet<String> = input.split_whitespace().map(String::from).collect();


                        println!("Type space separated words to insert into set B:");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let b:HashSet<String> = input.split_whitespace().map(String::from).collect();

                        print_results(a,b);
                        break;
                    }
                    break;
                }

                2 => {
                    loop{
                        println!("\nType space separated integer values to insert into set A:");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let a: HashSet<i32> = input
                            .split_whitespace() //split the content up into pieces between spaces
                            .map(|s| s.parse::<i32>()) // convert &str to int
                            .collect::<Result<HashSet<i32>, _>>() 
                            .expect("Found non integer values!");


                        println!("Type space separated integer values to insert into set B:");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let b: HashSet<i32> = input
                            .split_whitespace() //split the content up into pieces between spaces
                            .map(|s| s.parse::<i32>()) // convert &str to int
                            .collect::<Result<HashSet<i32>, _>>() 
                            .expect("Found non integer values!");

                        print_results(a, b);
                        break;
                    }
                    break;
                }
                _ => {
                    println!{"Not a valid user input."};
                    continue;
                }
            }
        }
    }

    
    Ok(())
}

fn get_int_input() -> u64 {
    loop{
        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // If the user inputs nothing, exit the input step
        if input.trim().is_empty() {
            println!("You didn't input anything!\n");
            continue;
        }

        // Check if input is a valid integer
        match input.trim().parse::<u64>(){
            Ok(num) => return num,
            Err(_) => println!("That was not a valid positive number!"),
        }
    }

}

// Universal function that prints any type hashset
fn print_results<T: std::fmt::Debug + Eq + Hash + Clone>(a: HashSet<T>, b: HashSet<T>){
    println!("Set A: {a:?}");
    println!("Set B: {b:?}");
    println!("A ∪ B: {:?}", a.union(&b).collect::<Vec<_>>());
    println!("A ∩ B: {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("A / B: {:?}", a.difference(&b).collect::<Vec<_>>());
    println!("B / A: {:?}", b.difference(&a).collect::<Vec<_>>()); 
    println!("A Δ B: {:?}", a.symmetric_difference(&b));
}