use std::ops::Rem;
use std::thread;
use std::env;
use std::thread::JoinHandle;

fn main() {
    let x: u128 = env::args().nth(1).unwrap_or(20.to_string()).parse().unwrap();
    let threads: u128 = env::args().nth(2).unwrap_or(2.to_string()).parse().unwrap();
    let mut hi: Vec<JoinHandle<bool>> = vec![];

    for i in 0..threads {
        let a: u128 = x.clone();
        let c: u128 = (i+1) * a.clone() / threads;
        let b: u128;
        if i > 0 {
            b = i * a.clone() / threads;
        } else {
            b = 2;
        }
        hi.push(thread::spawn(move || {
            let is_prime: bool = is_prime(a, b, c);
            return is_prime;
        }))
    }
    let mut is_prime: bool = true;
    for thread in hi {
        let hi2 = thread.join().unwrap();
        if !hi2 {
            is_prime = false;
            break;
        }
    }
    println!("{} is prime: {}", x, is_prime)
}

fn is_prime(x: u128, y: u128, z: u128) -> bool {
    for i in y..z {
        if x.rem(i) == 0 {
            return false
        }
    }
    true
}