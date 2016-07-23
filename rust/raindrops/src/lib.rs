extern crate primal;

use primal::Sieve;

pub fn raindrops(x: usize) -> String {
    let sieve_up_to = 2 << 16;
    let sieve = Sieve::new(sieve_up_to);
    let factors_as_tuples = sieve.factor(x).unwrap();
    let factors: Vec<usize> = factors_as_tuples.iter().map( |tuple| tuple.0 ).collect();
   
    let mut result = "".to_string();
    let mut found = false;
    if factors.iter().any(|&factor| factor == 3) {
        result = result + "Pling";
        found = true;
    }

    if factors.iter().any(|&factor| factor == 5) {
        result = result + "Plang";
        found = true;
    }

    if factors.iter().any(|&factor| factor == 7) {
        result = result + "Plong";
        found = true;
    }

    match found {
        true => result,
        false => x.to_string()
    }
}
