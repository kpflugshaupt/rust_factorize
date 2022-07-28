use pyo3::prelude::*;

use factorize::{Number, prime_factors};

pub mod factorize;

/// Decompose a positive number into its constituent prime factors.
#[pyfunction]
//#[pyo3(text_signature = "(number: int) -> list[int]")]
fn to_primes(number: Number) -> PyResult<Vec<Number>> {
    Ok(prime_factors(number))
}

/// Python module holding the to_primes() function.
#[pymodule]
fn rust_factorize(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_primes, m)?)?;
    Ok(())
}
