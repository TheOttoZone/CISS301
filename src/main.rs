use std::io;

const MAX_VARIABLES: u8 = 4;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}

/*
1. Get input from the user
    1a. Check how many variables the user is using, if more than 4 + Function bit, exit
    1b. If an input line has a 1 in the rightmost bit, store it for later   <---- do this after sorting probably
    1c. Establish an exit statement or character to stop the input of truth table lines
2. Check to see if input values are in numerical order (implement some sort algorithm ig)
3. Start printing values.
    3a. Print initial pretty table stuff
    3b. Check if first value starts at 0, if not print 0.
    3c. If difference between currently printing value and previous is greater than 1, print previous + 1
    3d. When printing values, it would probably make sense to print like print {value} >> 1 to then place a | between that and the last number 
    (maybe do like an if {value} && 1 == 1 then print 1)
4. Take previously stored values and print them with logic converting them to /ABC + form.
 */ 