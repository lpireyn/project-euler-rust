#![allow(dead_code)]

use std::ops::{Mul, Rem};

#[rustfmt::skip]
const SOLUTIONS: &[fn()] = &[
    solution1,
    solution2,
    unimplemented,
    unimplemented,
    solution5,
    solution6,
];

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert!(
        // args[0] is the binary name
        args.len() == 2,
        "Exactly one parameter expected: problem number"
    );
    let number = args[1].parse::<usize>().expect("Invalid problem number");
    let solution = SOLUTIONS[number - 1];
    solution();
}

fn solution1() {
    let sum = (0..1000)
        .filter(|n| is_multiple(*n, 3) || is_multiple(*n, 5))
        .sum::<i32>();
    println!("{sum}");
    assert_eq!(sum, 233168);
}

fn solution2() {
    let mut p2 = 1;
    let mut p1 = 2;
    let mut sum = p1;
    loop {
        let n = p2 + p1;
        if n >= 4_000_000 {
            break;
        }
        if is_even(n) {
            sum += n;
        }
        p2 = p1;
        p1 = n;
    }
    println!("{sum}");
    assert_eq!(sum, 4613732);
}

fn solution5() {
    let ds = &[11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut n = 21u64;
    loop {
        if ds.iter().all(|d| is_multiple(n, d)) {
            break;
        }
        n += 1;
    }
    println!("{n}");
    assert_eq!(n, 232792560);
}

fn solution6() {
    const MAX: u32 = 100;
    let sum_of_squares = (1..=MAX).map(sqr).sum::<u32>();
    let square_of_sum = sqr((1..=MAX).sum::<u32>());
    let diff = square_of_sum as i32 - sum_of_squares as i32;
    println!("{diff}");
    assert_eq!(diff, 25164150);
}

#[allow(unused)]
fn unimplemented() {
    panic!("Solution not yet implemented");
}

fn sqr<N>(n: N) -> N::Output
where
    N: Mul + Copy,
{
    n * n
}

fn is_multiple<N, D>(n: N, d: D) -> bool
where
    N: Rem<D, Output = N> + PartialEq + From<u8>,
{
    n % d == 0u8.into()
}

fn is_odd<N>(n: N) -> bool
where
    N: Rem<Output = N> + PartialEq + From<u8>,
{
    !is_multiple(n, 2u8.into())
}

fn is_even<N>(n: N) -> bool
where
    N: Rem<Output = N> + PartialEq + From<u8>,
{
    is_multiple(n, 2u8.into())
}
