const SOLUTIONS: &[fn()] = &[];

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

#[allow(unused)]
fn unimplemented() {
    panic!("Solution not yet implemented");
}
