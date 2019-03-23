use pyo3::class::*;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;

use std::path::Path;

use syntect::parsing::SyntaxReference as _SyntaxReference;
use syntect::parsing::SyntaxSet as _SyntaxSet;
use syntect::parsing::SyntaxSetBuilder as _SyntaxSetBuilder;

use crate::scope::Scope;
use crate::syntax_definition::SyntaxDefinition;

// -------- SyntaxSetBuilder --------
#[pyclass]
pub struct SyntaxSetBuilder {
    pub wrap: _SyntaxSetBuilder,
}

#[pymethods]
impl SyntaxSetBuilder {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(SyntaxSetBuilder {
            wrap: _SyntaxSetBuilder::new(),
        });
    }

    fn build(&self) -> SyntaxSet {
        return SyntaxSet {
            wrap: self.wrap.clone().build(),
        };
    }

    fn add(&mut self, syntax: &SyntaxDefinition) {
        self.wrap.add(syntax.wrap.clone());
    }

    fn add_plain_text_syntax(&mut self) {
        self.wrap.add_plain_text_syntax();
    }

    fn add_from_folder(&mut self, path: &str, lines_include_newline: bool) -> PyResult<()> {
        let result = self
            .wrap
            .add_from_folder(Path::new(path), lines_include_newline);

        if result.is_err() {
            Err(exceptions::Exception::py_err(
                "SyntaxSetBuilder.add_from_folder",
            ))
        } else {
            Ok(())
        }
    }
}

// -------- SyntaxReference --------
#[pyclass]
pub struct SyntaxReference {
    pub wrap: _SyntaxReference,
}

#[pymethods]
impl SyntaxReference {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.wrap.name.clone())
    }

    #[getter]
    fn scope(&self) -> PyResult<Scope> {
        Ok(Scope {
            wrap: self.wrap.scope.clone(),
        })
    }

    #[getter]
    fn file_extensions(&self) -> PyResult<Vec<String>> {
        Ok(self.wrap.file_extensions.clone())
    }
}

// -------- SyntaxSet --------
#[pyclass]
pub struct SyntaxSet {
    pub wrap: _SyntaxSet,
}

#[pyproto]
impl PySequenceProtocol for SyntaxSet {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.wrap.syntaxes().len())
    }

    fn __getitem__(&self, key: isize) -> PyResult<SyntaxReference> {
        Ok(SyntaxReference {
            wrap: self.wrap.syntaxes()[(key as usize)].clone(),
        })
    }
}

#[pymethods]
impl SyntaxSet {
    #[classmethod]
    fn load_defaults_newlines(_cls: &PyType) -> SyntaxSet {
        return SyntaxSet {
            wrap: _SyntaxSet::load_defaults_newlines(),
        };
    }

    #[classmethod]
    fn load_defaults_nonewlines(_cls: &PyType) -> SyntaxSet {
        return SyntaxSet {
            wrap: _SyntaxSet::load_defaults_nonewlines(),
        };
    }

    fn find_syntax_by_name(&self, name: &str) -> PyResult<SyntaxReference> {
        let result = self.wrap.find_syntax_by_name(name);
        match result {
            Some(x) => Ok(SyntaxReference { wrap: x.clone() }),
            None => Err(exceptions::Exception::py_err(
                "SyntaxSet.find_syntax_by_name",
            )),
        }
    }

    fn find_syntax_by_token(&self, s: &str) -> PyResult<SyntaxReference> {
        let result = self.wrap.find_syntax_by_token(s);
        match result {
            Some(x) => Ok(SyntaxReference { wrap: x.clone() }),
            None => Err(exceptions::Exception::py_err(
                "SyntaxSet.find_syntax_by_token",
            )),
        }
    }

    fn find_syntax_for_file(&self, path: &str) -> PyResult<SyntaxReference> {
        let result = self.wrap.find_syntax_for_file(Path::new(path)).unwrap();
        match result {
            Some(x) => Ok(SyntaxReference { wrap: x.clone() }),
            None => Err(exceptions::Exception::py_err(
                "SyntaxSet.find_syntax_for_file",
            )),
        }
    }

    fn find_syntax_by_extension(&self, extension: &str) -> PyResult<SyntaxReference> {
        let result = self.wrap.find_syntax_by_extension(extension);
        match result {
            Some(x) => Ok(SyntaxReference { wrap: x.clone() }),
            None => Err(exceptions::Exception::py_err(
                "SyntaxSet.find_syntax_by_extension",
            )),
        }
    }

    fn find_syntax_by_first_line(&self, s: &str) -> PyResult<SyntaxReference> {
        let result = self.wrap.find_syntax_by_first_line(s);
        match result {
            Some(x) => Ok(SyntaxReference { wrap: x.clone() }),
            None => Err(exceptions::Exception::py_err(
                "SyntaxSet.find_syntax_by_first_line",
            )),
        }
    }

    fn find_syntax_by_path(&self, path: &str) -> PyResult<SyntaxReference> {
        let result = self.wrap.find_syntax_by_path(path);
        match result {
            Some(x) => Ok(SyntaxReference { wrap: x.clone() }),
            None => Err(exceptions::Exception::py_err(
                "SyntaxSet.find_syntax_by_path",
            )),
        }
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<SyntaxSetBuilder>();
    m.add_class::<SyntaxReference>();
    m.add_class::<SyntaxSet>();
}
