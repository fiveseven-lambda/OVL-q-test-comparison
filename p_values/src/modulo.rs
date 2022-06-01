/*
Copyright (c) 2022 Atsushi Komaba

This file is part of OVL-test.

OVL-test is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

OVL-test is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with OVL-test.  If not, see <https://www.gnu.org/licenses/>.
*/

pub fn add(a: u32, b: u32, modulus: u32) -> u32 {
    if a < modulus - b {
        a + b
    } else {
        a - (modulus - b)
    }
}

#[test]
fn test_add() {
    let modulus = u32::MAX;
    assert_eq!(add(0, 0, modulus), 0);
    assert_eq!(add(0, modulus - 1, modulus), modulus - 1);
    assert_eq!(add(2, modulus - 1, modulus), 1);
    assert_eq!(add(modulus - 1, 0, modulus), modulus - 1);
    assert_eq!(add(modulus - 1, 2, modulus), 1);
    assert_eq!(add(modulus - 1, modulus - 1, modulus), modulus - 2);
}

pub fn sub(a: u32, b: u32, modulus: u32) -> u32 {
    if a >= b {
        a - b
    } else {
        a + (modulus - b)
    }
}

#[test]
fn test_sub() {
    let modulus = u32::MAX;
    assert_eq!(sub(0, 1, modulus), modulus - 1);
    assert_eq!(sub(1, 0, modulus), 1);
    assert_eq!(sub(modulus - 1, 0, modulus), modulus - 1);
    assert_eq!(sub(0, modulus - 1, modulus), 1);
    assert_eq!(sub(modulus - 1, modulus - 2, modulus), 1);
    assert_eq!(sub(modulus - 2, modulus - 1, modulus), modulus - 1);
}

pub fn mul(a: u32, b: u32, modulus: u32) -> u32 {
    (a as u64 * b as u64 % modulus as u64) as u32
}

pub fn div(a: u32, b: u32, modulus: u32) -> u32 {
    mul(a, inv(b, modulus), modulus)
}

pub fn pow(mut x: u32, mut n: u32, modulus: u32) -> u32 {
    let mut ret = 1;
    while n != 0 {
        if n % 2 != 0 {
            mul_assign(x, &mut ret, modulus);
        }
        mul_assign(x, &mut x, modulus);
        n /= 2;
    }
    ret
}

#[test]
fn test_pow() {
    for modulus in [1_000_000_007] {
        assert_eq!(pow(0, 0, modulus), 1);
        assert_eq!(pow(0, 1, modulus), 0);
        assert_eq!(pow(0, 2, modulus), 0);

        for x in 1..modulus.min(100) {
            let mut ans = 1;
            for n in 0..100 {
                assert_eq!(pow(x, n, modulus), ans);
                mul_assign(x, &mut ans, modulus);
            }
        }
    }
}

pub fn inv(x: u32, modulus: u32) -> u32 {
    let mut a = (modulus, 0);
    let mut b = (x, 1);
    let mut sign = false;
    while b.0 != 0 {
        a.1 += b.1 * (a.0 / b.0);
        a.0 %= b.0;
        sign = !sign;
        std::mem::swap(&mut a, &mut b);
    }
    if sign {
        a.1
    } else {
        modulus - a.1
    }
}

#[test]
fn test_inv() {
    let n = 1000;
    for modulus in (0..)
        .filter(|&p| primal::is_prime(p.into()))
        .take(10)
        .chain(
            (0..)
                .map(|i| u32::MAX - i)
                .filter(|&p| primal::is_prime(p.into()))
                .take(10),
        )
    {
        for n in 1..modulus.min(n + 1) {
            let inv = inv(n, modulus);
            assert_eq!(mul(n, inv, modulus), 1);
        }
        for n in modulus.max(n + 1) - n..modulus {
            let inv = inv(n, modulus);
            assert_eq!(mul(n, inv, modulus), 1);
        }
    }
}

// pub fn add_assign(src: u32, dest: &mut u32, modulus: u32) {
//     *dest = add(*dest, src, modulus);
// }

// pub fn sub_assign(src: u32, dest: &mut u32, modulus: u32) {
//     *dest = sub(*dest, src, modulus);
// }

pub fn mul_assign(src: u32, dest: &mut u32, modulus: u32) {
    *dest = mul(*dest, src, modulus);
}

pub fn div_assign(src: u32, dest: &mut u32, modulus: u32) {
    *dest = div(*dest, src, modulus);
}
