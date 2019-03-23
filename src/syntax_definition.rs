use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;
use std::collections::HashMap;

use syntect::parsing::syntax_definition::Context as _Context;
use syntect::parsing::syntax_definition::SyntaxDefinition as _SyntaxDefinition;

use crate::scope::Scope;

// -------- Context --------
#[pyclass]
pub struct Context {
    pub wrap: _Context,
}

#[pymethods]
impl Context {
    #[getter]
    fn meta_scope(&self) -> PyResult<Vec<Scope>> {
        let mut result = Vec::new();
        for x in self.wrap.meta_scope.clone() {
            result.push(Scope { wrap: x });
        }
        Ok(result)
    }

    // #[getter]
    // fn meta_content_scope(&self) -> PyResult<Vec<Scope>> {
    //     Ok(self.wrap.meta_content_scope)
    // }

    // #[getter]
    // fn meta_include_prototype(&self) -> PyResult<bool> {
    //     Ok(self.wrap.meta_include_prototype)
    // }

    // #[getter]
    // fn uses_backrefs(&self) -> PyResult<bool> {
    //     Ok(self.wrap.uses_backrefs)
    // }
}

// -------- SyntaxDefinition --------
#[pyclass]
pub struct SyntaxDefinition {
    pub wrap: _SyntaxDefinition,
}
#[pymethods]
impl SyntaxDefinition {
    #[classmethod]
    fn load_from_str(
        _cls: &PyType,
        s: &str,
        lines_include_newline: bool,
        fallback_name: Option<&str>,
    ) -> PyResult<SyntaxDefinition> {
        let result = _SyntaxDefinition::load_from_str(s, lines_include_newline, fallback_name);

        if result.is_err() {
            Err(exceptions::Exception::py_err(
                "SyntaxDefinition.load_from_str",
            ))
        } else {
            Ok(SyntaxDefinition {
                wrap: result.unwrap(),
            })
        }
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.wrap.name.clone())
    }

    #[getter]
    fn variables(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.wrap.clone().variables)
    }

    #[getter]
    fn scope(&self) -> PyResult<Scope> {
        Ok(Scope {
            wrap: self.wrap.scope,
        })
    }

    #[getter]
    fn file_extensions(&self) -> PyResult<Vec<String>> {
        Ok(self.wrap.file_extensions.clone())
    }

    // #[getter]
    // fn contexts(&self) -> PyResult<HashMap<String, Context>> {
    //     let mut res: HashMap<String, Context> = HashMap::new();
    //     Ok(res)
    //     // Ok(self.wrap.contexts)
    // }

    #[getter]
    fn hidden(&self) -> PyResult<bool> {
        Ok(self.wrap.hidden)
    }

    // #[new]
    //  fn new(
    //      obj: &PyRawObject,
    //      name: String, file_extensions: Vec<String>,
    //      scope: Scope, first_line_match: Option<String>,
    //      hidde: bool,
    //      variables: HashMap<String, String>,
    //      contexts: HashMap<String, Context>
    //  ) {
    //      obj.init(SyntaxDefinition {
    //          name,
    //          file_extensions,
    //          scope,
    //          first_line_match,
    //          hidden,
    //          variables,
    //          contexts,
    //      });
    //  }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<SyntaxDefinition>();
}
