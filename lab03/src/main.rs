use std::io;
use std::io::Write;

fn main(){

    loop{
        println!("\nChoose a calculation to do:
1. Factorial of n
2. Fibonacci sequence of n numbers
3. Pi calculated to n digits up to 15
4. Quit
----------------------------------------");

        // call input function
        let number = get_input();

        // match user input to 1 of 4 values
        match number {
            1 => {
                print!("\nType a number to find the factorial of: ");
                // flush stdout to actually print
                io::stdout().flush().unwrap();
                let fac_num = get_input();

                // calc and print
                println!("\n{}! = {}", fac_num, factorial(fac_num));
            }

            2 => {
                print!("\nType a number to calculate the fibonacci sequence to: ");
                io::stdout().flush().unwrap();
                let fib_num = get_input();

                println!("\nFibonacci {} = {}", fib_num, fibonacci(fib_num));
            }

            3 => {
                print!("\nType a number of digits to calculate pi to (up to 15): ");
                io::stdout().flush().unwrap();
                let mut pi_num = get_input();

                // im too lazy to code higher capacity stuff
                if pi_num > 15{
                    println!("\nInput larger than 15, printing to only 15 digits.");
                    pi_num = 15;
                }
                println!("\nPi to {} digits = {}", pi_num, find_pi(pi_num));
            }

            4 => break,
            
            _ => {
                println!("\nNot a valid user input.");
                continue;
            }
        }
    }
}

fn get_input() -> u64 {
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

// factorial algorithm from https://programming-idioms.org/idiom/31/recursive-factorial-simple/450/rust 
fn factorial(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

// fibonacci sequence algorithm from https://benjaminbrandt.com/fibonacci-in-rust/
fn fibonacci(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 0;
    let mut count = 0;
    
    while count < n {
        let tmp = a + b;
        b = a;
        a = tmp;
        count += 1;
    }
    
    b
}

// https://en.wikipedia.org/wiki/Gauss%E2%80%93Legendre_algorithm 
fn find_pi(n: u64) -> f64{
    let mut a = 1.0;
    let mut b = 1.0 / 2.0_f64.sqrt();
    let mut p = 1.0;
    let mut t = 1.0/4.0;

    for _ in 0..10{
        let a_next = (a + b)/ 2.0;
        let b_next = (a * b).sqrt();
        let p_next = 2.0 * p;
        let t_next = t - p * (a_next - a).powf(2.0);

        a = a_next;
        b = b_next;
        p = p_next;
        t = t_next;
    }

    // final computation
    let pi = ((a+b).powf(2.0))/(4.0 * t);

    // trim result to inputted amount
    let result = f64::trunc(pi * (10.0_f64.powf(n as f64))) / (10.0_f64.powf(n as f64));
    result
}