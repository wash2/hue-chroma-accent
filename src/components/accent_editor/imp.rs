// SPDX-License-Identifier: MPL-2.0-only

use adw::{
    gtk::{glib, subclass::prelude::*, Box, ColorButton, CssProvider, Switch, ToggleButton},
    StyleManager,
};
use once_cell::sync::OnceCell;
use std::{cell::RefCell, rc::Rc};

// Object holding the state
#[derive(Default)]
pub struct AccentEditor {
    pub css_provider: Rc<OnceCell<CssProvider>>,
    pub color_editor: Rc<OnceCell<Box>>,
    pub accent_button: Rc<OnceCell<ColorButton>>,
    pub use_palette_switch: Rc<OnceCell<Switch>>,
    pub palette_buttons: Rc<RefCell<Vec<ToggleButton>>>,
    pub palette_box: Rc<OnceCell<Box>>,
    pub style_manager: Rc<OnceCell<StyleManager>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for AccentEditor {
    const NAME: &'static str = "AccentEditorWidget";
    type Type = super::AccentEditor;
    type ParentType = Box;
}

// Trait shared by all GObjects
impl ObjectImpl for AccentEditor {}

// Trait shared by all widgets
impl WidgetImpl for AccentEditor {}

// Trait shared by all boxes
impl BoxImpl for AccentEditor {}
