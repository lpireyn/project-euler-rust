use std::ops::{Add, Mul, Rem};

pub fn sqr<N, N2>(n: N) -> N2
where
    N2: From<N> + Mul<Output = N2> + Copy,
{
    let n2 = N2::from(n);
    n2 * n2
}

pub fn sqr_u8(n: u8) -> u16 {
    sqr(n)
}

pub fn sqr_u16(n: u16) -> u32 {
    sqr(n)
}

pub fn sqr_u32(n: u32) -> u64 {
    sqr(n)
}

pub fn is_multiple<N, D>(n: N, d: D) -> bool
where
    N: Rem<D, Output = N> + PartialEq + From<u8>,
{
    n % d == 0u8.into()
}

pub fn is_odd<N>(n: N) -> bool
where
    N: Rem<Output = N> + PartialEq + From<u8>,
{
    !is_multiple(n, 2u8.into())
}

pub fn is_even<N>(n: N) -> bool
where
    N: Rem<Output = N> + PartialEq + From<u8>,
{
    is_multiple(n, 2u8.into())
}

pub fn is_pythagorean_triplet<N, N2>(a: N, b: N, c: N) -> bool
where
    N2: From<N> + PartialEq + Add<Output = N2> + Mul<Output = N2> + Copy,
{
    sqr::<N, N2>(a) + sqr::<N, N2>(b) == sqr::<N, N2>(c)
}

pub fn is_pythagorean_triplet_u16(a: u16, b: u16, c: u16) -> bool {
    is_pythagorean_triplet::<u16, u32>(a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pythagorean_triplet() {
        assert!(is_pythagorean_triplet::<u8, u8>(3, 4, 5));
    }
}
