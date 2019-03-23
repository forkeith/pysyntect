#![allow(unused_imports)]

use pyo3::prelude::*;
use pyo3::types::PyType;

use syntect::easy::HighlightLines as _HighlightLines;
use syntect::easy::ScopeRegionIterator as _ScopeRegionIterator;

use crate::scope::ScopeStackOp;
use crate::style::Style;
use crate::syntax_set::SyntaxReference;
use crate::syntax_set::SyntaxSet;
use crate::theme::Theme;

// -------- HighlightLines --------
#[pyclass]
pub struct HighlightLines {
    pub wrap: _HighlightLines<'static>,
}

#[pymethods]
impl HighlightLines {
    #[new]
    fn new(obj: &PyRawObject, syntax: &SyntaxReference, theme: &'static Theme) {
        obj.init(HighlightLines {
            wrap: _HighlightLines::new(&syntax.wrap, &theme.wrap),
        });
    }

    fn highlight(&mut self, line: &str, syntax_set: &SyntaxSet) -> Vec<(Style, String)> {
        let mut res = Vec::new();
        let x = self.wrap.highlight(line, &syntax_set.wrap);
        for &tpl in &x {
            res.push((Style { wrap: tpl.0 }, tpl.1.to_string()));
        }
        return res;
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<HighlightLines>();
}
