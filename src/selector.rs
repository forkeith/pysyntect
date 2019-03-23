use pyo3::class::*;
use pyo3::prelude::*;
use pyo3::types::PyType;

use std::str::FromStr;

use syntect::highlighting::ScopeSelector as _ScopeSelector;
use syntect::highlighting::ScopeSelectors as _ScopeSelectors;

use crate::scope::MatchPower;
use crate::scope::Scope;
use crate::scope::ScopeStack;

// -------- ScopeSelectors --------
#[pyclass]
pub struct ScopeSelectors {
    pub wrap: _ScopeSelectors,
}

#[pyproto]
impl PyObjectProtocol for ScopeSelectors {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
}

#[pyproto]
impl PySequenceProtocol for ScopeSelectors {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.wrap.selectors.len())
    }

    fn __getitem__(&self, key: isize) -> PyResult<ScopeSelector> {
        Ok(ScopeSelector {
            wrap: self.wrap.selectors[(key as usize)].clone(),
        })
    }
}

#[pymethods]
impl ScopeSelectors {
    fn does_match(&self, stack: &ScopeStack) -> PyResult<Option<MatchPower>> {
        let result = self.wrap.does_match(&stack.wrap.as_slice());
        match result {
            Some(x) => Ok(Some(MatchPower { wrap: x })),
            None => Ok(None),
        }
    }

    #[classmethod]
    fn from_str(_cls: &PyType, s: &str) -> ScopeSelectors {
        return ScopeSelectors {
            wrap: _ScopeSelectors::from_str(s).unwrap(),
        };
    }
}

// -------- ScopeSelector --------
#[pyclass]
pub struct ScopeSelector {
    pub wrap: _ScopeSelector,
}

#[pyproto]
impl PyObjectProtocol for ScopeSelector {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
}

#[pymethods]
impl ScopeSelector {
    fn does_match(&self, stack: &ScopeStack) -> PyResult<Option<MatchPower>> {
        let result = self.wrap.does_match(&stack.wrap.as_slice());
        match result {
            Some(x) => Ok(Some(MatchPower { wrap: x })),
            None => Ok(None),
        }
    }

    fn extract_single_scope(&self) -> PyResult<Option<Scope>> {
        let result = self.wrap.extract_single_scope();
        match result {
            Some(x) => Ok(Some(Scope { wrap: x })),
            None => Ok(None),
        }
    }

    #[classmethod]
    fn from_str(_cls: &PyType, s: &str) -> ScopeSelector {
        return ScopeSelector {
            wrap: _ScopeSelector::from_str(s).unwrap(),
        };
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<ScopeSelectors>();
    m.add_class::<ScopeSelector>();
}
