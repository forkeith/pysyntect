use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::Path;
use syntect::html::highlighted_html_for_file as _highlighted_html_for_file;
use syntect::html::highlighted_html_for_string as _highlighted_html_for_string;

use crate::syntax_set::SyntaxReference;
use crate::syntax_set::SyntaxSet;
use crate::theme::Theme;

// -------- highlighted_html_for_string --------
#[pyfunction]
fn highlighted_html_for_string(
    s: &str,
    ss: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &Theme,
) -> PyResult<String> {
    let result = _highlighted_html_for_string(s, &ss.wrap, &syntax.wrap, &theme.wrap);
    Ok(result)
}

// -------- highlighted_html_for_file --------
#[pyfunction]
fn highlighted_html_for_file(path: &str, ss: &SyntaxSet, theme: &Theme) -> PyResult<String> {
    let result = _highlighted_html_for_file(Path::new(path), &ss.wrap, &theme.wrap);

    if result.is_err() {
        Err(exceptions::Exception::py_err("highlighted_html_for_file"))
    } else {
        Ok(result.unwrap())
    }
}

pub fn initialize(m: &PyModule) {
    m.add_wrapped(wrap_pyfunction!(highlighted_html_for_file))
        .unwrap();
    m.add_wrapped(wrap_pyfunction!(highlighted_html_for_string))
        .unwrap();
}
