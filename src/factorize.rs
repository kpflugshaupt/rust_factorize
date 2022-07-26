//! Take a positive integer number as argument, calculate its prime factor decomposition

/// Initial number of factor slots in result
const FACTOR_SLOTS: usize = 20;

/// Type of source number and factors
pub type Number = u64;

/// Calculate floor of square root
fn int_sqrt(nr: Number) -> Number {
    unsafe { (nr as f64).sqrt().to_int_unchecked() }
}

/// Split `number` >= 4 into prime factors by optimized "direct testing"
fn factorize(number: Number) -> Vec<Number> {
    let mut remainder = number;
    let mut factors: Vec<Number> = Vec::with_capacity(FACTOR_SLOTS);
    let factor_limit = int_sqrt(number.clone()) + 1;
    // extract 2, then the rest, skipping multiples of 2
    while remainder % 2 == 0 {
        factors.push(2);
        remainder /= 2;
    }
    let mut factor: Number = 3;
    while factor <= factor_limit && remainder > 1 {
        while remainder % factor == 0 {
            factors.push(factor);
            remainder /= factor;
        }
        factor += 2;
    }
    // last remainder is another factor, if > 1
    if remainder > 1 {
        factors.push(remainder)
    }
    factors
}

/// Calculate the prime factors making up a number, sorted by size.
///
/// # Examples
///
/// ```
/// use rust_factorize::prime_factors;
///
/// let fact = prime_factors(0);
/// assert_eq!(fact, vec![]);
///
/// let fact = prime_factors(12);
/// assert_eq!(fact, vec![2, 2, 3]);
///
/// let fact = prime_factors(1_000_000_000);
/// assert_eq!(fact, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
/// ```
pub fn prime_factors(number: Number) -> Vec<Number> {
    match number {
        0 | 1 => vec![],
        2 | 3 => vec![number],
        _ => factorize(number),
    }
}

#[cfg(test)]
mod tests {
    use crate::factorize::*;

    #[test]
    fn empty_results() {
        for value in [0, 1] {
            assert_eq!(prime_factors(value), vec![])
        }
    }

    #[test]
    fn single_results() {
        for value in [2, 3, 5, 11] {
            assert_eq!(prime_factors(value), vec![value]);
        }
    }

    #[test]
    fn multi_results() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(25), vec![5, 5]);
        assert_eq!(prime_factors(62), vec![2, 31]);
        assert_eq!(
            prime_factors(1_000_000_000),
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5]
        );
    }
}
