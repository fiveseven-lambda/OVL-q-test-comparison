use crate::bit::UnsignedInt;
use crate::modulo;
use crate::number_theoretic_transform::Prime;

/// binom(2n, n) * (1 - p<sub>q,n,n</sub>(i / n)) for each q = 1, 2 and i = 0, 1, ..., n - 1
pub fn p_values_modulo(n: u32, prime: &Prime) -> Vec<[u32; 2]> {
    let bit = n.bit_width();
    let un = n as usize;

    // fibonacci polynomials F_i for i = 0, 1, ..., 2n - 1
    let fibs: Vec<_> = (0..n * 2)
        .map(|i| fibonacci_polynomial(i, prime.value()))
        .collect();

    let mut p1_series = fibs[0..un].to_vec();
    for (denom, numer) in fibs[1..].iter().step_by(2).zip(&mut p1_series) {
        prime.polymul(numer.clone(), numer);
        numer.truncate(un + 1);
        let mut inv_denom = prime.polyinv(denom.clone(), bit);
        inv_denom.truncate(un + 1);
        prime.polymul(inv_denom, numer);
    }
    let p1_values = p1_series.iter().map(|p| p[un]);

    let mut p2_series = fibs[1..=un + 1].to_vec();
    for (denom, numer) in fibs.into_iter().zip(&mut p2_series) {
        prime.polydiff(numer);
        let mut inv_denom = prime.polyinv(denom, bit);
        inv_denom.truncate(un + 1);
        prime.polymul(inv_denom, numer);
    }
    let p2_values = p2_series
        .windows(2)
        .map(|p| modulo::sub(p[0][un], p[1][un], prime.value()));

    p1_values
        .zip(p2_values)
        .rev()
        .map(|(x, y)| [x, y])
        .collect()
}

fn fibonacci_polynomial(n: u32, modulus: u32) -> Vec<u32> {
    (0..=n / 2)
        .scan(1, |t, i| {
            let ret = *t;
            if i < n / 2 {
                modulo::mul_assign(n - i * 2, t, modulus);
                modulo::mul_assign(n - i * 2 - 1, t, modulus);
                modulo::div_assign(i + 1, t, modulus);
                modulo::div_assign(n - i, t, modulus);
            }
            Some(if i % 2 == 0 {
                ret
            } else {
                modulo::sub(0, ret, modulus)
            })
        })
        .collect()
}
