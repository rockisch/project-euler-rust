mod lib;
mod vars;

use std::collections::HashMap;

fn main() {
    println!("{:?}", ex10(2_000_000));
}

#[allow(unused)]
fn ex10(n: i32) -> u64 {
    let mut primes: Vec<i32> = vec![2];
    'outer: for i in (3..n).step_by(2) {
        for p in &primes {
            if i % p == 0 {
                continue 'outer
            }
        }
        primes.push(i);
    }
    primes.iter()
        .map(|p| *p as u64)
        .sum()
}

#[allow(unused)]
fn ex9(n: i32) -> i32 {
    for a in 1..n {
        for b in a..n {
            let c = 1000 - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                println!("{} {} {}", a, b, c);
                return a*b*c;
            }
        }
    };
    -1
}

#[allow(unused)]
fn ex8(n: i32) -> u64 {
    let mut max: u64 = 0;
    let numbers: Vec<u64> = vars::EX8_NUMBERS
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    for i in numbers.windows(13) {
        let product: u64 = i.iter().product();
        if max < product { max = product }
    }
    max
}

#[allow(unused)]
fn ex7(n: usize) -> i32 {
    let mut primes = vec![2];
    let mut i = 2;
    'outer: loop {
        i += 1;
        for p in &primes {
            if i % p == 0 {
                continue 'outer
            }
        }
        primes.push(i);
        if primes.len() > n { break; };
    }
    primes[n - 1]
}

#[allow(unused)]
fn ex6(n: i32) -> i32 {
    let mut squ_sum = 0;
    let mut nat_sum = 0;
    for i in 0..n + 1 {
        squ_sum += i.pow(2);
        nat_sum += i;
    }
    let squ_nat_sum = nat_sum.pow(2);
    squ_nat_sum - squ_sum
}

#[allow(unused)]
fn ex5(n: i32) -> i32 {
    let mut result: i32 = 1;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..n {
        let factors = lib::factors(i).unwrap();
        let mut factor_map: HashMap<i32, i32> = HashMap::new();
        for f in factors {
            let entry = factor_map.entry(f).or_insert(0);
            *entry += 1;
        }
        for (k, v) in factor_map.iter() {
            match map.get(k) {
                Some(n) => if v > n { map.insert(*k, *v); },
                None => { map.insert(*k, *v); },
            }
        }
    }
    for (k, v) in map.iter() {
        result *= k.pow(*v as u32);
    }
    result
}

#[allow(unused)]
fn ex4(n: usize) -> i32 {
    let mut max: i32 = 0;
    let loop_max: i32 = "9".repeat(n).parse().unwrap();
    let loop_min: i32 = ("1".to_string() + &"0".repeat(n-1)).parse().unwrap();
    for x in (loop_min..loop_max + 1).rev() {
        for y in (loop_min..loop_max + 1).rev() {
            let mult = x * y;
            if mult < max { break; }
            let val = mult.to_string();
            let half = val.len();
            if val.len() % 2 == 0 {
                if val.bytes().take(half).eq(val.bytes().rev().take(half)) {
                    max = val.parse::<i32>().unwrap();
                }
            }
        }
    }
    max
}

#[allow(unused)]
fn ex3(n: i32) -> i32 {
    let mut factors = lib::factors(n).unwrap();
    factors.pop().unwrap()
}

#[allow(unused)]
fn ex2(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut x: i32 = 1;
    let mut y: i32 = 0;
    while x < n {
        let z = x;
        x = y + x;
        y = z;
        if x % 2 == 0 {
            sum += x
        }
    }
    sum
}

#[allow(unused)]
fn ex1(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..n {
        if (i % 3 == 0) | (i % 5 == 0) {
            sum += i;
        }
    }
    sum
}
