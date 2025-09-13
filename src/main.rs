#![allow(dead_code)]

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

fn is_multiple(n: i32, d: i32) -> bool {
    n % d == 0
}

fn is_odd(n: i32) -> bool {
    !is_multiple(n, 2)
}

fn is_even(n: i32) -> bool {
    is_multiple(n, 2)
}
