mod fib;

use fib::Fibonacci;

fn ex1(n: i32) -> i32 {
    (1..n)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |x, y| x + y)
}

fn ex2(n: i32) -> i32 {
    Fibonacci::new()
        .take_while(|x| x <= &n)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn ex3(n: i64) -> i64 {
    let mut mut_n = n;
    let mut max: i64 = 1;
    for i in 2..(n as f64).sqrt() as i64 {
        while mut_n % i == 0 {
            mut_n /= i;
            max = i;
        }
    }
    max
}

fn ex4(n: usize) -> i32 {
    let mut max = 0;
    let n_min = (1.to_string() + &0.to_string().repeat(n - 1)).parse::<i32>().unwrap();
    let n_max = n_min * 10;

    for x in (n_min..n_max).rev() {
        for y in (x..n_max).rev() {
            let prod = x * y;
            if y == n_max - 1 && prod < max { return max }
            let mut prod_string = prod.to_string();
            if prod_string.len() % 2 == 0 {
                let prod_rev = prod_string.split_off(prod_string.len() / 2).chars().rev().collect::<String>();
                if prod_string == prod_rev && max < prod {
                    max = prod;
                }
            }
        }
    }
    max
}

fn ex5(n: i32) -> i32 {
    let mut primes = Vec::new();
    for i in 2..n + 1 {
        if primes.iter().all(|x| i % x != 0) {
            primes.push(i);
        } else {
            let mut pv = Vec::new();
            let mut mut_i = i;
            for p in primes.iter() {
                while mut_i % p == 0 {
                    pv.push(p);
                }
            }
        }
    }
    primes.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    macro_rules! timed_test {
        ( $f:ident($v:expr), $n:expr ) => {
            let start = Instant::now();
            assert_eq!($f($v), $n);
            let duration = start.elapsed();
            
            println!("Time elapsed in {:?} is: {:?}", stringify!($f), duration);
        };
    }

    #[test]
    fn test_ex1() {
        timed_test!(ex1(1000), 233168);
    }

    #[test]
    fn test_ex2() {
        timed_test!(ex2(4000000), 4613732);
    }

    #[test]
    fn test_ex3() {
        timed_test!(ex3(600851475143 as i64), 6857 as i64);
    }

    #[test]
    fn test_ex4() {
        timed_test!(ex4(3), 906609);
    }

    #[test]
    fn test_ex5() {
        timed_test!(ex5(20), 232792560);
    }
}