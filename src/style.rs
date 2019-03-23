use pyo3::class::*;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::PyResult;

use std::string::String;
use syntect::highlighting::Color as _Color;
use syntect::highlighting::FontStyle as _FontStyle;
use syntect::highlighting::Style as _Style;
use syntect::highlighting::StyleModifier as _StyleModifier;

// -------- Color --------
#[pyclass]
pub struct Color {
    pub wrap: _Color,
}

#[pyproto]
impl PyObjectProtocol for Color {
    fn __richcmp__(&self, other: &Color, op: CompareOp) -> PyResult<bool> {
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

impl ToPyObject for Color {
    fn to_object(&self, py: Python) -> PyObject {
        Color { wrap: self.wrap }.into_object(py)
    }
}

#[pymethods]
impl Color {
    #[new]
    fn new(obj: &PyRawObject, r: u8, g: u8, b: u8, a: u8) {
        obj.init(Color {
            wrap: _Color { r, g, b, a },
        });
    }

    #[getter]
    fn r(&self) -> PyResult<u8> {
        Ok(self.wrap.r)
    }
    #[getter]
    fn g(&self) -> PyResult<u8> {
        Ok(self.wrap.g)
    }
    #[getter]
    fn b(&self) -> PyResult<u8> {
        Ok(self.wrap.b)
    }
    #[getter]
    fn a(&self) -> PyResult<u8> {
        Ok(self.wrap.a)
    }
}

// -------- FontStyle --------
#[pyclass]
pub struct FontStyle {
    pub wrap: _FontStyle,
}

impl ToPyObject for FontStyle {
    fn to_object(&self, py: Python) -> PyObject {
        FontStyle { wrap: self.wrap }.into_object(py)
    }
}

#[pyproto]
impl PyObjectProtocol for FontStyle {
    fn __richcmp__(&self, other: &FontStyle, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Err(exceptions::ValueError::py_err("< not implemented")),
            CompareOp::Le => Err(exceptions::ValueError::py_err("<= not implemented")),
            CompareOp::Eq => Ok(self.wrap == other.wrap),
            CompareOp::Ne => Ok(self.wrap != other.wrap),
            CompareOp::Gt => Err(exceptions::ValueError::py_err("> not implemented")),
            CompareOp::Ge => Err(exceptions::ValueError::py_err(">= not implemented")),
        }
    }
}

// -------- Style --------
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[pyclass]
#[derive(Hash)]
pub struct Style {
    pub wrap: _Style,
}

impl ToPyObject for Style {
    fn to_object(&self, py: Python) -> PyObject {
        Style { wrap: self.wrap }.into_object(py)
    }
}

#[pyproto]
impl PyObjectProtocol for Style {
    fn __richcmp__(&self, other: &Style, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Err(exceptions::ValueError::py_err("< not implemented")),
            CompareOp::Le => Err(exceptions::ValueError::py_err("<= not implemented")),
            CompareOp::Eq => Ok(self.wrap == other.wrap),
            CompareOp::Ne => Ok(self.wrap != other.wrap),
            CompareOp::Gt => Err(exceptions::ValueError::py_err("> not implemented")),
            CompareOp::Ge => Err(exceptions::ValueError::py_err(">= not implemented")),
        }
    }

    fn __hash__(&self) -> PyResult<isize> {
        Ok(my_hash(self.wrap) as isize)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.wrap))
    }
}

#[pymethods]
impl Style {
    #[getter]
    fn foreground(&self) -> PyResult<Color> {
        Ok(Color {
            wrap: self.wrap.foreground,
        })
    }

    #[getter]
    fn background(&self) -> PyResult<Color> {
        Ok(Color {
            wrap: self.wrap.background,
        })
    }

    #[getter]
    fn font_style(&self) -> PyResult<FontStyle> {
        Ok(FontStyle {
            wrap: self.wrap.font_style,
        })
    }
}

// -------- StyleModifier --------
#[pyclass]
pub struct StyleModifier {
    pub wrap: _StyleModifier,
}

impl ToPyObject for StyleModifier {
    fn to_object(&self, py: Python) -> PyObject {
        StyleModifier { wrap: self.wrap }.into_object(py)
    }
}

#[pymethods]
impl StyleModifier {
    #[getter]
    fn foreground(&self) -> PyResult<Color> {
        Ok(Color {
            wrap: self.wrap.foreground.unwrap(),
        })
    }

    #[getter]
    fn background(&self) -> PyResult<Color> {
        Ok(Color {
            wrap: self.wrap.background.unwrap(),
        })
    }

    #[getter]
    fn font_style(&self) -> PyResult<FontStyle> {
        Ok(FontStyle {
            wrap: self.wrap.font_style.unwrap(),
        })
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<Style>();
    m.add_class::<StyleModifier>();
    m.add_class::<Color>();
    m.add_class::<FontStyle>();
}
