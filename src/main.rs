#[rustfmt::skip]
const SOLUTIONS: &[fn()] = &[
    solution1,
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

#[allow(unused)]
fn unimplemented() {
    panic!("Solution not yet implemented");
}

fn is_multiple(n: i32, d: i32) -> bool {
    n % d == 0
}
