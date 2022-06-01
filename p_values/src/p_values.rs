use crate::bit::UnsignedInt;
use crate::number_theoretic_transform::Prime;
use crate::p_values_modulo::p_values_modulo;
use num::{BigRational, BigUint, One, ToPrimitive, Zero};
use std::sync::mpsc;
use std::thread;
use itertools::Itertools;

pub fn p_values(n: u32) -> Vec<[f64; 2]> {
    let bit = n.bit_width() + 1;
    let denom = big_binom(n * 2, n);
    let un = n as usize;
    let mut numers: Vec<[BigUint; 2]> = vec![[Zero::zero(), Zero::zero()]; un + 1];

    let primes = (0..1 << (31 - bit))
        .rev()
        .filter_map(|i| Prime::from_coef_exp(i, bit))
        .scan(BigUint::one(), |product, prime| {
            (*product < denom).then(|| {
                *product *= BigUint::from(prime.value());
                prime
            })
        });
    for chunk in &primes.chunks(32) {
        let chunk: Vec<_> = chunk.collect();
        let chunk_size = chunk.len();
        let (sender, receiver) = mpsc::channel();
        for prime in chunk {
            let sender = sender.clone();
            thread::spawn(move || {
                let result = p_values_modulo(n, &prime);
                sender.send((prime.value().into(), result)).unwrap();
            });
        }
        let mut modulus: BigUint = One::one();
        for _ in 0..chunk_size {
            let (p, result) = receiver.recv().unwrap();
            for (row_mod, row) in result.into_iter().zip(numers.iter_mut()) {
                for (numer_mod, numer) in row_mod.into_iter().zip(row) {
                    *numer += sub_mod(numer_mod.into(), &*numer % &p, &p) * inv_mod(&modulus, &p) % &p
                        * &modulus;
                }
            }
            modulus *= &p;
            eprintln!("{}%", modulus.bits() as f64 / denom.bits() as f64 * 100.);
        }
    }
    numers
        .into_iter()
        .map(|row| {
            row.map(|numer| {
                BigRational::new((&denom - numer).into(), denom.clone().into())
                    .to_f64()
                    .unwrap()
            })
        })
        .collect()
}

fn big_binom(n: u32, r: u32) -> BigUint {
    if r == 0 {
        return One::one();
    }
    use std::collections::VecDeque;
    fn product(mut queue: VecDeque<BigUint>) -> BigUint {
        loop {
            let x = queue.pop_front().unwrap();
            if let Some(y) = queue.pop_front() {
                queue.push_back(x * y);
            } else {
                break x;
            }
        }
    }
    let numer = product((n - r + 1..=n).map(Into::into).collect());
    let denom = product((1..=r).map(Into::into).collect());
    numer / denom
}

fn inv_mod(value: &BigUint, modulus: &BigUint) -> BigUint {
    value.modpow(&(modulus - 2u32), modulus)
}

fn sub_mod(a: BigUint, b: BigUint, modulus: &BigUint) -> BigUint {
    if a >= b {
        a - b
    } else {
        a + (modulus - b)
    }
}
