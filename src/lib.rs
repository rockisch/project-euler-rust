extern crate num;

use std::convert::TryFrom;
use std::error::Error;

use num::Integer;

pub fn factors<T>(n: T) -> Result<Vec<T>, T::Error>
where
    T: Integer + Copy + TryFrom<i32>,
{
    let mut factors: Vec<T> = Vec::new();
    let mut n: T = n;
    let mut d: T = T::try_from(2)?;
    while n > T::try_from(1)? {
        while n % d == T::try_from(0)? {
            factors.push(d);
            n = n / d;
        }
        d = d + T::try_from(1)?;
        if d * d > n {
            if n > T::try_from(1)? {
                factors.push(n);
            }
            break;
        }
    }
    return Ok(factors);
}

pub fn primes_until<T>(n: T) -> Result<Vec<T>, Box<Error>>
where
    T: Integer + Copy + std::fmt::Debug + TryFrom<i32>,
    u8: Into<T>,
{
    let mut primes = vec![2u8.into()];
    let mut i = 2u8.into();
    'outer: loop {
        i = i + 1u8.into();
        for p in &primes {
            if i % *p == 0u8.into() {
                continue 'outer
            }
        }
        primes.push(i);
        println!("{:?}", i);
        if (primes.len() as u8).into() > n { break; };
    }
    Ok(primes)
}
