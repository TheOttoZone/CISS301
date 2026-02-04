use std::io;

fn main(){
    println!("{}", find_pi(100));
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


fn find_pi(n: usize) -> f64{
    let mut a = 1.0;
    let mut b = 1.0 / 2.0_f64.sqrt();
    let mut p = 1.0;
    let mut t = 1.0/4.0;

    for _ in 0..n{
        let a_next = (a + b)/ 2.0;
        let b_next = (a * b).sqrt();
        let p_next = 2.0 * p;
        let t_next = t - p * (a_next - a).powi(2);

        a = a_next;
        b = b_next;
        p = p_next;
        t = t_next;
    }

    let pi = ((a+b).powi(2))/(4.0 * t);
    pi
}