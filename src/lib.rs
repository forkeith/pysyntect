#![allow(unused_must_use)]

use pyo3::prelude::*;
use pyo3::{PyResult, Python};

mod easy;
mod html;
mod parser;
mod scope;
mod selector;
mod style;
mod syntax_definition;
mod syntax_set;
mod theme;
mod theme_set;

#[pymodule]
fn pysyntect(_py: Python, m: &PyModule) -> PyResult<()> {
    easy::initialize(m);
    html::initialize(m);
    parser::initialize(m);
    scope::initialize(m);
    selector::initialize(m);
    style::initialize(m);
    syntax_definition::initialize(m);
    syntax_set::initialize(m);
    theme::initialize(m);
    theme_set::initialize(m);

    Ok(())
}
