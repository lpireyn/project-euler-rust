#![allow(dead_code)]

use std::ops::Rem;

#[rustfmt::skip]
const SOLUTIONS: &[fn()] = &[
    solution1,
    solution2,
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

#[allow(unused)]
fn unimplemented() {
    panic!("Solution not yet implemented");
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
