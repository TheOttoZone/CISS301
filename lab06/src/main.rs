use regex::Regex;
use std::env;
use std::fs::{File, read_to_string};
fn main() {
    let args: Vec<String> = env::args().collect();

    let date_regex = Regex::new(r"(?<date>\d{1,2}\/\d{1,2}\/\d{4})").unwrap();
    let time_regex = Regex::new(r"(?<time>\d{2}:\d{2}:\d{2})").unwrap();
    let ip_regex = Regex::new(r"(?<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
    /*if args.len() != 2 {
        eprintln!("You must enter in two files to be parsed!");
        eprintln!("The first file should be a csv containing people information, the second containing network logs.");
        return;
    }*/
    let raw_network_text = read_to_string(&args[1].clone()).expect("Failed to read file");
    //let raw_people_text = read_to_string(args[2].clone()).expect("Failed to read file");
    
    for line in raw_network_text.lines(){
        if let Some(caps) = date_regex.captures(&line) {
            print!("Date: {}, ", &caps["date"]);
        }
        if let Some(caps) = time_regex.captures(&line) {
            print!("Time: {}, ", &caps["time"]);
        }
        if let Some(caps) = ip_regex.captures(&line) {
            print!("IP: {}\n", &caps["ip"]);
        }
    }

}
