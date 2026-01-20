use std::io;

const MAX_VARIABLES: usize = 4;

fn main() {
    let mut table: Vec<u8> = Vec::new();
    let mut pre_function: Vec<u8> = Vec::new();
    let mut num_variables: usize = 0;
    let mut input = String::new();

    // Let the user know whats goin on
    println!("Input a truth table, enter nothing to continue:");

    loop {
        // 1. Get input from the user
        input.clear();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        // Trim the intput to allow the it to be parsed properly
        let trimmed = input.trim();

        // If the user inputs nothing, exit the input step
        if trimmed.is_empty() {
            break;
        }

        // 1a. Check how many variables the user is using, if more than 4 + Function bit, exit
        if num_variables == 0 {
            // Set the number of variables to the number of digits in the input value disregarding the function bit
            num_variables = trimmed.chars().count().saturating_sub(1);

            if num_variables < 2 {
                println!("Input is too short!")
            }
            
            if num_variables > MAX_VARIABLES{
            println!("Input is too long!");
            break;
            }
            
        }else {
            // Check for mismatched variable count
            if trimmed.chars().count().saturating_sub(1) != num_variables{
                println!("Number entered is not the same size as the rest of the table!");
                break;
            }
        }
        // Convert the input to a binary value to be stored
        match u8::from_str_radix(input.trim(), 2){
            Ok(result) => {
                // Only push to the vectors if the input does not already exist in the vector in either form
                let result2 = result ^ 1;
                if table.contains(&result){
                    println!("You have already input this exact value!")
                } else if table.contains(&result2){
                    println!("You have already input this value!")
                }else{
                    table.push(result);
                    // 1b. If an input line has a 1 in the rightmost bit, store it for later
                    if result & 1 == 1{
                        pre_function.push(result);
                    }
                }

            }
            Err(_) => println!("Not a valid binary number!")
        }
    }
    
    //println!("You entered a {num_variables} digit number");
    //println!("blah: {:?}", pre_function);

    // 2. Check to see if input values are in numerical order (implement some sort algorithm ig)
    // Silly past me, rust has a sort built in!
    table.sort_by_key(|&num| num >> 1);
    pre_function.sort_by_key(|&num| num >> 1);
    
    println!("\nTruth table:");

    // Call display function
    display(table, num_variables);

    println!("Logic Expression (Sum of Products):\n");

    // Logic time!
    expressionalize(pre_function, num_variables);

}


fn display(table: Vec<u8>, num_variables: usize){
    let mut last: u8 = 0;
    // Change what we are printing ever so slightly to account for the variable number of variables
    match num_variables {
        2 => {
            // 3a. Print initial pretty table stuff
            println!("AB|F\n----");
            for num in &table {
                // 3b. Check if first value starts at 0, if not print 0.
                if table[0] as i32 != 0 {
                    println!("00|0");
                } 
                // 3c. If difference between currently printing value and previous is greater than 1, print previous + 1
                for missing_value in (last + 1)..num >> 1{
                    println!("{:02b}|0 ", missing_value);
                };
                last = num >> 1;
                // 3d. Actually print the currently observed table value
                println!("{:02b}|{:b} ", num >> 1, num & 1);
            }
        }
        3 => {
            println!("ABC|F\n-----");
            for num in &table {
                if table[0] as i32 != 0 {
                    println!("000|0");
                } 
                for missing_value in (last + 1)..num >> 1{
                    println!("{:03b}|0 ", missing_value);
                };
                last = num >> 1;
                println!("{:03b}|{:b} ", num >> 1, num & 1);
            }
        }
        4 => {
            println!("ABCD|F\n----");
            for num in &table {
                if table[0] as i32 != 0 {
                    println!("0000|0");
                } 
                for missing_value in (last + 1)..num >> 1{
                    println!("{:04b}|0 ", missing_value);
                };
                last = num >> 1;
                println!("{:04b}|{:b} ", num >> 1, num & 1);
            }
        }
        _ => println!("Oops!"),
    }
    println!("\n\n")
}

// 4. Take previously stored values and print them with logic converting them to /ABC + form.
fn expressionalize(pre_function: Vec<u8>, num_variables: usize){
    let letters = ['A', 'B', 'C', 'D'];
    // Track length so we can determine when we are at the end of the vector
    let len = pre_function.len();
    for (i, num) in pre_function.iter().enumerate(){
        for x in 0..num_variables{
            // If the currently observed variable is 0, print a / representing NOT
            if (num >> (num_variables - x)) & 1 == 0{
                print!("/")
            }
            // Print the letter that corresponds to the currently observed variable
            print!("{}", letters[x]);
        }
        // Print a plus if we are not at the end of the vector and there is actually more to look at
        if i != len - 1{
            print!(" + ");
        }        
    }
    println!("\n");
}
/*
Program Goals:

Done! 1. Get input from the user
Done! 1a. Check how many variables the user is using, if more than 4 + Function bit, exit 
Done! 1b. If an input line has a 1 in the rightmost bit, store it for later   <---- do this after sorting probably
Done! 1c. Establish an exit statement or character to stop the input of truth table lines
Done! 2. Check to see if input values are in numerical order (implement some sort algorithm ig)
3. Start printing values.
Done! 3a. Print initial pretty table stuff
Done! 3b. Check if first value starts at 0, if not print 0.
Done! 3c. If difference between currently printing value and previous is greater than 1, print previous + 1
Done! 3d. When printing values, it would probably make sense to print like print {value} >> 1 to then place a | between that and the last number 
    (maybe do like an if {value} && 1 == 1 then print 1) WRONG
Done! 4. Take previously stored values and print them with logic converting them to /ABC + form.
 */ 