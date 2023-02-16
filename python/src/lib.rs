use md_ulb_pwrap::{ulb_wrap_paragraph};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "ulb_wrap_paragraph")]
fn py_ulb_wrap_paragraph(text: &str, width: usize, first_line_width: usize) -> PyResult<String> {
    Ok(ulb_wrap_paragraph(text, width, first_line_width))
}

#[pymodule]
#[pyo3(name = "md_ulb_pwrap")]
fn py_md_ulb_pwrap(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_ulb_wrap_paragraph, m)?)?;
    Ok(())
}
