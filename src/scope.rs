use pyo3::class::*;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyType;

use std::str::FromStr;

use syntect::parsing::MatchPower as _MatchPower;
use syntect::parsing::Scope as _Scope;
use syntect::parsing::ScopeRepository as _ScopeRepository;
use syntect::parsing::ScopeStack as _ScopeStack;
use syntect::parsing::ScopeStackOp as _ScopeStackOp;

// -------- Scope --------
#[pyclass]
pub struct Scope {
    pub wrap: _Scope,
}
impl ToPyObject for Scope {
    fn to_object(&self, py: Python) -> PyObject {
        Scope {
            wrap: self.wrap.clone(),
        }
        .into_object(py)
    }
}
#[pyproto]
impl PyObjectProtocol for Scope {
    fn __richcmp__(&self, other: &Scope, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Err(exceptions::ValueError::py_err("< not implemented")),
            CompareOp::Le => Err(exceptions::ValueError::py_err("<= not implemented")),
            CompareOp::Eq => Ok(self.wrap == other.wrap),
            CompareOp::Ne => Ok(self.wrap != other.wrap),
            CompareOp::Gt => Err(exceptions::ValueError::py_err("> not implemented")),
            CompareOp::Ge => Err(exceptions::ValueError::py_err(">= not implemented")),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
}
#[pymethods]
impl Scope {
    #[new]
    fn new(obj: &PyRawObject, s: &str) {
        obj.init(Scope {
            wrap: _Scope::new(s).unwrap(),
        });
    }

    #[classmethod]
    fn from_str(_cls: &PyType, s: &str) -> PyResult<Scope> {
        let result = _Scope::from_str(s);
        if result.is_err() {
            Err(exceptions::Exception::py_err("Scope.from_str exception"))
        } else {
            Ok(Scope {
                wrap: result.unwrap(),
            })
        }
    }

    fn is_prefix_of(&self, s: &Scope) -> bool {
        return self.wrap.is_prefix_of(s.wrap);
    }
}

// -------- ScopeRepository --------
#[pyclass]
pub struct ScopeRepository {
    pub wrap: _ScopeRepository,
}
#[pymethods]
impl ScopeRepository {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(ScopeRepository {
            wrap: _ScopeRepository::new(),
        });
    }

    fn build(&mut self, s: &str) -> PyResult<Scope> {
        let result = self.wrap.build(s);
        if result.is_err() {
            Err(exceptions::Exception::py_err(
                "ScopeRepository.build exception",
            ))
        } else {
            Ok(Scope {
                wrap: result.unwrap(),
            })
        }
    }

    fn to_string(&mut self, scope: &Scope) -> String {
        return self.wrap.to_string(scope.wrap);
    }
}

// -------- ScopeStack --------
#[pyclass]
pub struct ScopeStack {
    pub wrap: _ScopeStack,
}
#[pyproto]
impl PyObjectProtocol for ScopeStack {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.wrap))
    }
}
#[pyproto]
impl PySequenceProtocol for ScopeStack {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.wrap.len())
    }
}
#[pymethods]
impl ScopeStack {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(ScopeStack {
            wrap: _ScopeStack::new(),
        });
    }

    #[classmethod]
    fn from_str(_cls: &PyType, s: &str) -> PyResult<ScopeStack> {
        let result = _ScopeStack::from_str(s);

        if result.is_err() {
            Err(exceptions::Exception::py_err(
                "ScopeStack.from_str exception",
            ))
        } else {
            Ok(ScopeStack {
                wrap: result.unwrap(),
            })
        }
    }

    fn is_empty(&mut self) -> bool {
        return self.wrap.is_empty();
    }

    fn push(&mut self, s: &Scope) {
        self.wrap.push(s.wrap);
    }

    fn pop(&mut self) {
        self.wrap.pop();
    }

    fn apply(&mut self, op: &ScopeStackOp) {
        self.wrap.apply(&op.wrap);
    }
}

// -------- ScopeStackOp --------
#[pyclass]
pub struct ScopeStackOp {
    pub wrap: _ScopeStackOp,
}
impl ToPyObject for ScopeStackOp {
    fn to_object(&self, py: Python) -> PyObject {
        ScopeStackOp {
            wrap: self.wrap.clone(),
        }
        .into_object(py)
    }
}
#[pyproto]
impl PyObjectProtocol for ScopeStackOp {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
}
#[pymethods]
impl ScopeStackOp {
    #[classmethod]
    fn noop(_cls: &PyType) -> ScopeStackOp {
        return ScopeStackOp {
            wrap: _ScopeStackOp::Noop,
        };
    }
}

// -------- MatchPower --------
#[pyclass]
pub struct MatchPower {
    pub wrap: _MatchPower,
}
#[pyproto]
impl PyObjectProtocol for MatchPower {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
}
#[pymethods]
impl MatchPower {
    #[new]
    fn new(obj: &PyRawObject, v: f64) {
        obj.init(MatchPower {
            wrap: _MatchPower(v),
        });
    }

    fn value(&self) -> f64 {
        return self.wrap.0;
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<Scope>();
    m.add_class::<ScopeRepository>();
    m.add_class::<ScopeStack>();
    m.add_class::<ScopeStackOp>();
    m.add_class::<MatchPower>();
}
