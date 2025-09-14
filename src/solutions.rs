#![allow(dead_code)]

use crate::{
    collatz::collatz,
    math::{is_even, is_multiple, is_palindromic, is_pythagorean_triplet_u16, sqr_u16},
    primes::primes,
};

fn solution1() -> u32 {
    (0u32..1000)
        .filter(|n| is_multiple(*n, 3) || is_multiple(*n, 5))
        .sum()
}

fn solution2() -> u32 {
    let mut p2 = 1u32;
    let mut p1 = 2u32;
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
    sum
}

fn solution3() -> u64 {
    todo!()
}

fn solution4() -> u32 {
    let mut max_product = 0u32;
    for i in 1u32..=999 {
        for j in 1u32..=999 {
            let product = i * j;
            if is_palindromic(product) && product > max_product {
                max_product = product;
            }
        }
    }
    max_product
}

// TODO: Too slow!
fn solution5() -> u64 {
    let ds = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut n = 40u64;
    while !ds.iter().all(|d| is_multiple(n, *d)) {
        n += 1;
    }
    n
}

fn solution6() -> u32 {
    const MAX: u16 = 100;
    let sum_of_squares: u32 = (1..=MAX).map(sqr_u16).sum();
    let square_of_sum: u32 = sqr_u16((1..=MAX).sum());
    square_of_sum - sum_of_squares
}

fn solution7() -> usize {
    primes(200_000)[10_001 - 1]
}

fn solution8() -> u64 {
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
    max_product
}

fn solution9() -> u64 {
    const SUM: u16 = 1000;
    let mut product = 0u64;
    'outer: for a in 0u16..(SUM / 3 - 2) {
        for b in (a + 1)..((SUM - a) / 2 - 1) {
            debug_assert!(b > a);
            let c = SUM - a - b;
            debug_assert!(c > b);
            debug_assert_eq!(a + b + c, SUM);
            if is_pythagorean_triplet_u16(a, b, c) {
                product = a as u64 * b as u64 * c as u64;
                break 'outer;
            }
        }
    }
    product
}

fn solution10() -> usize {
    let primes = primes(2_000_000 - 1);
    primes.iter().copied().sum::<usize>()
}

fn solution11() -> u64 {
    todo!()
}

fn solution12() -> u64 {
    todo!()
}

fn solution13() -> u64 {
    todo!()
}

fn solution14() -> u64 {
    let mut winner = 0u64;
    let mut max_len = 0usize;
    for n in 1u64..1_000_000 {
        let len = collatz(n).count();
        if len > max_len {
            winner = n;
            max_len = len;
        }
    }
    winner
}

fn solution15() -> u64 {
    todo!()
}

fn solution16() -> u64 {
    todo!()
}

fn solution17() -> u64 {
    todo!()
}

fn solution18() -> u64 {
    todo!()
}

fn solution19() -> u64 {
    todo!()
}

fn solution20() -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn solution1() {
        assert_eq!(super::solution1(), 233168);
    }

    #[test]
    fn solution2() {
        assert_eq!(super::solution2(), 4613732);
    }

    #[test]
    #[ignore]
    fn solution3() {
        assert_eq!(super::solution3(), 6857);
    }

    #[test]
    fn solution4() {
        assert_eq!(super::solution4(), 906609);
    }

    #[test]
    fn solution5() {
        assert_eq!(super::solution5(), 232792560);
    }

    #[test]
    fn solution6() {
        assert_eq!(super::solution6(), 25164150);
    }

    #[test]
    fn solution7() {
        assert_eq!(super::solution7(), 104743);
    }

    #[test]
    fn solution8() {
        assert_eq!(super::solution8(), 23514624000);
    }

    #[test]
    fn solution9() {
        assert_eq!(super::solution9(), 31875000);
    }

    #[test]
    fn solution10() {
        assert_eq!(super::solution10(), 142913828922);
    }

    #[test]
    #[ignore]
    fn solution11() {
        assert_eq!(super::solution11(), 70600674);
    }

    #[test]
    #[ignore]
    fn solution12() {
        assert_eq!(super::solution12(), 76576500);
    }

    #[test]
    #[ignore]
    fn solution13() {
        assert_eq!(super::solution13(), 5537376230);
    }

    #[test]
    fn solution14() {
        assert_eq!(super::solution14(), 837799);
    }

    #[test]
    #[ignore]
    fn solution15() {
        assert_eq!(super::solution15(), 137846528820);
    }

    #[test]
    #[ignore]
    fn solution16() {
        assert_eq!(super::solution16(), 1366);
    }

    #[test]
    #[ignore]
    fn solution17() {
        assert_eq!(super::solution17(), 21124);
    }

    #[test]
    #[ignore]
    fn solution18() {
        assert_eq!(super::solution18(), 1074);
    }

    #[test]
    #[ignore]
    fn solution19() {
        assert_eq!(super::solution19(), todo!());
    }

    #[test]
    #[ignore]
    fn solution20() {
        assert_eq!(super::solution20(), 648);
    }
}
