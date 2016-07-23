extern crate primal;

use primal::Sieve;

pub fn raindrops(x: usize) -> String {
    let sieve_up_to = 2 << 16;
    let sieve = Sieve::new(sieve_up_to);
    let factors_as_tuples = sieve.factor(x).unwrap();
    let mut factors = factors_as_tuples.iter().map( |tuple| tuple.0 );
    let has3 = factors.any(|factor| factor == 3);
    if has3 {
        return "Pling".to_string();
    }
    return x.to_string();
}
