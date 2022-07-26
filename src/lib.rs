use pyo3::prelude::*;

use factorize::{prime_factors, Number};

pub mod factorize;

#[pyfunction]
fn to_primes(number: Number) -> PyResult<Vec<Number>> {
    Ok(prime_factors(number))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_factorize(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_primes, m)?)?;
    Ok(())
}
