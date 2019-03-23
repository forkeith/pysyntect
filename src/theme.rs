use pyo3::prelude::*;

use syntect::highlighting::Theme as _Theme;
use syntect::highlighting::ThemeItem as _ThemeItem;
use syntect::highlighting::ThemeSettings as _ThemeSettings;

use crate::selector::ScopeSelectors;
use crate::style::Color;
use crate::style::StyleModifier;

// -------- ThemeSettings --------
#[pyclass]
pub struct ThemeSettings {
    pub wrap: _ThemeSettings,
}

#[pymethods]
impl ThemeSettings {
    #[getter]
    fn accent(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.accent;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn active_guide(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.active_guide;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn background(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.background;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn bracket_contents_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.bracket_contents_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    // #[getter]
    // fn bracket_contents_options(&self) -> PyResult<Option<UnderlineOption>> {
    //     let result = self.wrap.bracket_contents_options;
    //     match result {
    //         Some(x) => Ok(Some(UnderlineOption {wrap: x})),
    //         None => Ok(None),
    //     }
    // }
    #[getter]
    fn brackets_background(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.brackets_background;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn brackets_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.brackets_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    // #[getter]
    // fn brackets_options(&self) -> PyResult<Option<UnderlineOption>> {
    //     let result = self.wrap.brackets_options;
    //     match result {
    //         Some(x) => Ok(Some(UnderlineOption {wrap: x})),
    //         None => Ok(None),
    //     }
    // }
    #[getter]
    fn caret(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.caret;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn find_highlight(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.find_highlight;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn find_highlight_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.find_highlight_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn guide(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.guide;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn gutter(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.gutter;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn gutter_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.gutter_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn highlight(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.highlight;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn highlight_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.highlight_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn inactive_selection(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.inactive_selection;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn inactive_selection_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.inactive_selection_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn line_highlight(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.line_highlight;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn minimap_border(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.minimap_border;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn misspelling(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.misspelling;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn phantom_css(&self) -> PyResult<Option<String>> {
        let result = self.wrap.phantom_css.clone();
        match result {
            Some(x) => Ok(Some(x)),
            None => Ok(None),
        }
    }
    #[getter]
    fn popup_css(&self) -> PyResult<Option<String>> {
        let result = self.wrap.popup_css.clone();
        match result {
            Some(x) => Ok(Some(x)),
            None => Ok(None),
        }
    }
    #[getter]
    fn selection(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.selection;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn selection_background(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.selection_background;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn selection_border(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.selection_border;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn selection_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.selection_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn shadow(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.shadow;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn stack_guide(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.stack_guide;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    #[getter]
    fn tags_foreground(&self) -> PyResult<Option<Color>> {
        let result = self.wrap.tags_foreground;
        match result {
            Some(x) => Ok(Some(Color { wrap: x })),
            None => Ok(None),
        }
    }
    // #[getter]
    // fn tags_options(&self) -> PyResult<Option<UnderlineOption>> {
    //     let result = self.wrap.tags_options;
    //     match result {
    //         Some(x) => Ok(Some(UnderlineOption {wrap: x})),
    //         None => Ok(None),
    //     }
    // }
}

// -------- Theme --------
impl ToPyObject for Theme {
    fn to_object(&self, py: Python) -> PyObject {
        Theme {
            wrap: self.wrap.clone(),
        }
        .into_object(py)
    }
}

// -------- ThemeItem --------
#[pyclass]
pub struct ThemeItem {
    pub wrap: _ThemeItem,
}

impl ToPyObject for ThemeItem {
    fn to_object(&self, py: Python) -> PyObject {
        ThemeItem {
            wrap: self.wrap.clone(),
        }
        .into_object(py)
    }
}

#[pymethods]
impl ThemeItem {
    #[getter]
    fn scope(&self) -> PyResult<ScopeSelectors> {
        Ok(ScopeSelectors {
            wrap: self.wrap.scope.clone(),
        })
    }
    #[getter]
    fn style(&self) -> PyResult<StyleModifier> {
        Ok(StyleModifier {
            wrap: self.wrap.style.clone(),
        })
    }
}

#[pyclass]
pub struct Theme {
    pub wrap: _Theme,
}

#[pymethods]
impl Theme {
    #[getter]
    fn settings(&self) -> PyResult<ThemeSettings> {
        Ok(ThemeSettings {
            wrap: self.wrap.settings.clone(),
        })
    }

    #[getter]
    fn scopes(&self) -> PyResult<Vec<ThemeItem>> {
        let mut result = Vec::new();
        for x in self.wrap.scopes.clone() {
            result.push(ThemeItem { wrap: x.clone() });
        }
        Ok(result)
    }

    #[getter]
    fn name(&self) -> PyResult<Option<String>> {
        Ok(self.wrap.name.clone())
    }

    #[getter]
    fn author(&self) -> PyResult<Option<String>> {
        Ok(self.wrap.author.clone())
    }
}

pub fn initialize(m: &PyModule) {
    m.add_class::<ThemeItem>();
    m.add_class::<ThemeSettings>();
    m.add_class::<Theme>();
}
