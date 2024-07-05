use pyo3::prelude::*;

mod bridge;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    bridge::AllocateAndPrint();
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn zaworld(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
