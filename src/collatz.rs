use crate::math::is_even;

pub fn collatz(n: u64) -> CollatzIterator {
    CollatzIterator { n }
}

#[derive(Debug)]
pub struct CollatzIterator {
    n: u64,
}

impl Iterator for CollatzIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        if n == 0 {
            return None;
        }
        self.n = if n == 1 {
            0
        } else if is_even(n) {
            n / 2
        } else {
            3 * n + 1
        };
        Some(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        assert_eq!(
            collatz(13).collect::<Vec<_>>(),
            vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
    }
}
