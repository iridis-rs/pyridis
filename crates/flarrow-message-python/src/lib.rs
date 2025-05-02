use pyo3::prelude::*;

#[pymodule]
fn pyflarrow_message(_: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
