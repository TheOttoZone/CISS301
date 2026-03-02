use regex::Regex;
use std::env;
use std::fs::read_to_string;
fn main() {
    let args: Vec<String> = env::args().collect(); // collect command line arguments

    // various regex definitions, probably a much better and condensed way to do this
    let date_regex = Regex::new(r"(?<date>\d{1,2}\/\d{1,2}\/\d{4})").unwrap();
    let time_regex = Regex::new(r"(?<time>\d{2}:\d{2}:\d{2})").unwrap();
    let ip_regex = Regex::new(r"(?<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
    let address_regex = Regex::new(r"(?<address>\d{1,5}\s.*?,)(?<city>.*?,)(?<state>.*?,)(?<country>.*?,)(?<zipcode>\d{4,5})").unwrap();
    let email_regex = Regex::new(r"(?<email>[\w\.-]+@[\w\.-]+\.\w+)").unwrap();
    let nanp_regex = Regex::new(r"(?<nanp>(\+1\s)?\(?\d{3}\)?[\s-]\d{3}[\s-]\d{4})").unwrap();
    let intern_regex = Regex::new(r"(?<intern>\+(\d{2,3}|[02-9])\s([\d \-]+\d))").unwrap();

    // if the amount of files supplied is not three, tell user they are silly and tell them how to do it right
    if args.len() != 4 {
        eprintln!("You must enter in three files to be parsed!");
        eprintln!("The first file should be a csv containing people information, the second containing network logs, and the third containing phone numbers in that specific order.");
        return;
    }

    // copy file data to string variables
    let raw_network_text = read_to_string(&args[1].clone()).expect("Failed to read file");
    let raw_people_text = read_to_string(&args[2].clone()).expect("Failed to read file");
    let raw_phone_text = read_to_string(&args[3].clone()).expect("Failed to read file");

    // read and parse network file and outpud desired data in a pretty way
    for line in raw_network_text.lines(){
        if let Some(caps) = date_regex.captures(&line) {
            print!("Date: {:13}", &caps["date"]); //13 here means to separate the Date: and the variable by 13 characters maybe idk
        }
        if let Some(caps) = time_regex.captures(&line) {
            print!("Time: {:13}", &caps["time"]);
        }
        if let Some(caps) = ip_regex.captures(&line) {
            print!("IP: {:13}\n", &caps["ip"]);
        }
    }

    // same for this guy
    for line in raw_people_text.lines(){
        if let Some(caps) = email_regex.captures(&line) {
            print!("Email: {:30}\t", &caps["email"])
        }
        if let Some(caps) = address_regex.captures(&line) {
            println!("Address: {} {} {} {}", &caps["address"], &caps["city"], &caps["state"], &caps["zipcode"])
        }
    }

    // here the logic kinda means more because the formatting can vary by line, so only one of these will output 
    for line in raw_phone_text.lines(){
        if let Some(caps) = nanp_regex.captures(&line) {
            println!("North American Phone Number:\t{}", &caps["nanp"])
        }
        if let Some(caps) = intern_regex.captures(&line) {
            println!("International Phone Number:\t{}", &caps["intern"])
        }
    }

}
