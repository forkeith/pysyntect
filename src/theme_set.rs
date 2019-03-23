use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyType;

use std::collections::HashMap;
use std::path::Path;
use syntect::highlighting::ThemeSet as _ThemeSet;

use crate::theme::Theme;

// -------- ThemeSet --------
#[pyclass]
pub struct ThemeSet {
    pub wrap: _ThemeSet,
}

#[pymethods]
impl ThemeSet {
    #[classmethod]
    fn load_defaults(_cls: &PyType) -> ThemeSet {
        return ThemeSet {
            wrap: _ThemeSet::load_defaults(),
        };
    }

    #[classmethod]
    fn load_from_folder(_cls: &PyType, path: &str) -> PyResult<ThemeSet> {
        let res = _ThemeSet::load_from_folder(Path::new(path));
        if res.is_err() {
            Err(exceptions::Exception::py_err("ThemeSet.load_from_folder"))
        } else {
            Ok(ThemeSet { wrap: res.unwrap() })
        }
    }

    #[getter]
    fn themes(&self) -> PyResult<HashMap<String, Theme>> {
        let mut res = HashMap::new();
        for theme in self.wrap.themes.clone() {
            res.insert(theme.0, Theme { wrap: theme.1 });
        }

        Ok(res)
    }

    #[getter]
    fn names(&self) -> PyResult<Vec<String>> {
        Ok(self.wrap.themes.keys().cloned().collect())
    }

    fn theme(&self, name: String) -> Theme {
        return Theme {
            wrap: self.wrap.themes[&name].clone(),
        };
    }

    #[classmethod]
    fn get_theme(_cls: &PyType, path: &str) -> PyResult<Theme> {
        let res = _ThemeSet::get_theme(Path::new(path));
        if res.is_err() {
            Err(exceptions::Exception::py_err("Theme.get_theme"))
        } else {
            Ok(Theme { wrap: res.unwrap() })
        }
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<ThemeSet>();
}
