#![allow(unused_imports)]

use pyo3::prelude::*;

use syntect::parsing::ParseState as _ParseState;

use crate::scope::ScopeStackOp;
use crate::syntax_set::SyntaxReference;
use crate::syntax_set::SyntaxSet;

// -------- ParseState --------
#[pyclass]
pub struct ParseState {
    pub wrap: _ParseState,
}

#[pymethods]
impl ParseState {
    #[new]
    fn new(obj: &PyRawObject, syntax: &SyntaxReference) {
        obj.init(ParseState {
            wrap: _ParseState::new(&syntax.wrap),
        });
    }

    fn parse_line(&mut self, line: &str, syntax_set: &SyntaxSet) -> Vec<(usize, ScopeStackOp)> {
        let mut res = Vec::new();
        for v in self.wrap.parse_line(line, &syntax_set.wrap) {
            res.push((v.0, ScopeStackOp { wrap: v.1 }));
        }
        return res;
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<ParseState>();
}
