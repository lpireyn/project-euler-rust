use crate::math::sqr_usize;

/// Return the prime numbers not greater than the given limit.
///
/// # Implementation
///
/// See [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).
pub fn primes(limit: usize) -> Vec<usize> {
    assert!(limit > 1, "invalid limit: {limit}");
    let mut primes = vec![true; limit - 1];
    let sqrt_limit = f64::sqrt(limit as f64) as usize;
    for i in 2..=sqrt_limit {
        if primes[i - 2] {
            // Optimization: start enumerating multiples of prime i from sqr(i)
            let mut j = sqr_usize(i);
            while j <= limit {
                primes[j - 2] = false;
                j += i;
            }
        }
    }
    primes
        .iter()
        .enumerate()
        .filter_map(|(index, prime)| if *prime { Some(index + 2) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_primes() {
        let first_100_primes = primes(1000).iter().take(100).copied().collect::<Vec<_>>();
        #[rustfmt::skip]
        assert_eq!(
            first_100_primes,
            // Source: https://prime-numbers.info/list/first-100-primes
            vec![
                2,   3,   5,   7,   11,  13,  17,  19,  23,  29,  // 1-10
                31,  37,  41,  43,  47,  53,  59,  61,  67,  71,  // 11-20
                73,  79,  83,  89,  97,  101, 103, 107, 109, 113, // 21-30
                127, 131, 137, 139, 149, 151, 157, 163, 167, 173, // 31-40
                179, 181, 191, 193, 197, 199, 211, 223, 227, 229, // 41-50
                233, 239, 241, 251, 257, 263, 269, 271, 277, 281, // 51-60
                283, 293, 307, 311, 313, 317, 331, 337, 347, 349, // 61-70
                353, 359, 367, 373, 379, 383, 389, 397, 401, 409, // 71-80
                419, 421, 431, 433, 439, 443, 449, 457, 461, 463, // 81-90
                467, 479, 487, 491, 499, 503, 509, 521, 523, 541  // 91-100
            ]
        );
    }
}
