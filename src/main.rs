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
    unimplemented,
    solution8,
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
    const MAX: u16 = 100;
    let sum_of_squares: u32 = (1..=MAX).map(sqr_u16).sum();
    let square_of_sum: u32 = sqr_u16((1..=MAX).sum());
    let diff = square_of_sum - sum_of_squares;
    println!("{diff}");
    assert_eq!(diff, 25164150);
}

fn solution8() {
    const SEQ_LEN: usize = 13;
    #[allow(clippy::char_lit_as_u8)]
    let digits = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".bytes().map(|b|(b - '0' as u8) as u64).collect::<Vec<_>>();
    debug_assert_eq!(digits.len(), 1000);
    let mut max_product = 0u64;
    for first in 0..(digits.len() - SEQ_LEN) {
        let product = digits[first..(first + SEQ_LEN)].iter().product();
        if product > max_product {
            max_product = product;
        }
    }
    println!("{max_product}");
    assert_eq!(max_product, 23514624000);
}

#[allow(unused)]
fn unimplemented() {
    panic!("Solution not yet implemented");
}

fn sqr<N, N2>(n: N) -> N2
where
    N2: From<N> + Mul<Output = N2> + Copy,
{
    let n2 = N2::from(n);
    n2 * n2
}

fn sqr_u8(n: u8) -> u16 {
    sqr(n)
}

fn sqr_u16(n: u16) -> u32 {
    sqr(n)
}

fn sqr_u32(n: u32) -> u64 {
    sqr(n)
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
