use num_bigint::BigUint;
use num_traits::{One, Zero};
use pyo3::prelude::*;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

#[pyfunction]
fn calc_fib(n: usize) -> PyResult<()> {
    let _ = fib(n);
    Ok(())
}

#[pymodule]
fn rust_fib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_fib, m)?)?;
    Ok(())
}
