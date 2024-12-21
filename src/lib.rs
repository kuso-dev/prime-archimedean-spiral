use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eratosthenes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=(n as f64).sqrt() as usize {
        if primes[i] {
            for j in (i * i..=n).step_by(i) {
                primes[j] = false;
            }
        }
    }

    primes
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| i)
        .collect()
}
