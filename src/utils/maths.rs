use std::mem::swap;

pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        if b < a {
            swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

pub fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}
