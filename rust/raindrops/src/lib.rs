extern crate primal;

use primal::Sieve;

pub fn raindrops(x: usize) -> String {
    let sieve_up_to = 2 << 16;
    let sieve = Sieve::new(sieve_up_to);
    let factors_as_tuples = sieve.factor(x).unwrap();
    let factors: Vec<usize> = factors_as_tuples.iter().map( |tuple| tuple.0 ).collect();
   
    if factors.iter().any(|&factor| factor == 3) {
        return "Pling".to_string();
    }

    if factors.iter().any(|&factor| factor == 5) {
        return "Plang".to_string();
    }

    if factors.iter().any(|&factor| factor == 7) {
        return "Plong".to_string();
    }

    return x.to_string();
}
