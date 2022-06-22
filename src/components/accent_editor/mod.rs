// SPDX-License-Identifier: MPL-2.0-only

use adw::gtk::Switch;
use adw::{
    builders::ExpanderRowBuilder, prelude::*, traits::ExpanderRowExt, ExpanderRow, StyleManager,
};
use adw::{
    gtk::{
        self,
        gdk::{self, RGBA},
        glib::{self, closure_local},
        subclass::prelude::*,
        Align, Box, Button, ColorButton, CssProvider, Entry, Label, MessageDialog, Orientation,
        ScrolledWindow, TextView, ToggleButton, Window,
    },
    ColorScheme,
};
use cascade::cascade;
use palette::{white_point::D65, IntoColor, Lch, Srgb};
use relm4_macros::view;
use std::fmt::Display;

use crate::{
    accent_palette,
    util::{self, SRGB},
};
mod imp;

glib::wrapper! {
    pub struct AccentEditor(ObjectSubclass<imp::AccentEditor>)
        @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AccentEditor {
    pub fn new(provider: CssProvider) -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create Theme Editor Widget");

        let imp = imp::AccentEditor::from_instance(&self_);

        cascade! {
            &self_;
            ..set_orientation(Orientation::Vertical);
        };

        let accent_color_button = cascade! {
            ColorButton::with_rgba(&&RGBA::new(0.0, 0.0, 0.0, 0.0));
            ..set_title("Accent Color");
            ..set_use_alpha(true);
        };

        view! {
            palette_box = &Box {
                set_orientation: Orientation::Horizontal,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,
                set_hexpand: true,
            }
        };

        let accented_button = Button::with_label("Demo Button");
        accented_button.add_css_class("suggested-action");

        view! {
            inner = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,
                set_hexpand: true,

                // Switch
                append: use_palette_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    set_hexpand: true,

                    append: use_palette_switch = &Switch {
                        set_active: true,
                    },

                    append: use_palette_label =  &Label {
                        set_text: "Use Palette",
                    },
                },

                // color picker
                append: color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    set_hexpand: true,

                    append: &accent_color_button,

                    append: label =  &Label {
                        set_text: "Accent Color",
                    },
                },
                // palette
                append: &palette_box,

                append: demo = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    set_hexpand: true,

                    append: &accented_button,
                    append: accent_color_label = &Label {
                        set_text: "Accent Color as text",
                        add_css_class: "accented-text",
                        add_css_class: "title-3"
                    },

                },
            }

        };

        let style_manager = adw::StyleManager::default();

        if style_manager.system_supports_color_schemes() {
            style_manager.connect_color_scheme_notify(
                glib::clone!(@weak self_ => move |style_manager| {
                    self_.set_palette_buttons(style_manager.is_dark());
                    self_.set_accent();
                }),
            );

            view! {
                color_scheme_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    set_hexpand: true,

                    append: system_color_scheme = &ToggleButton {
                        set_child: Some(&Label::new(Some("System Color Scheme"))),
                        set_active: true,
                        connect_toggled: glib::clone!(@weak style_manager => move |t| {
                            if t.is_active() {
                                style_manager.set_color_scheme(ColorScheme::Default);
                            }
                        })
                    },
                    append: light_color_scheme = &ToggleButton {
                        set_child: Some(&Label::new(Some("Light"))),
                        set_group: Some(&system_color_scheme),
                        connect_toggled: glib::clone!(@weak style_manager => move |t| {
                            if t.is_active() {
                                style_manager.set_color_scheme(ColorScheme::ForceLight);
                            }
                        })
                    },
                    append: dark_color_scheme = &ToggleButton {
                        set_child: Some(&Label::new(Some("Dark"))),
                        set_group: Some(&system_color_scheme),
                        connect_toggled: glib::clone!(@weak style_manager => move |t| {
                            if t.is_active() {
                                style_manager.set_color_scheme(ColorScheme::ForceDark);
                            }
                        })
                    }
                }
            };
            color_scheme_box.insert_after(&inner, Some(&palette_box));
        }

        let scroll_window = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&inner)
            .build();
        self_.append(&scroll_window);

        use_palette_switch.connect_state_set(glib::clone!(@weak self_, @weak imp.palette_buttons as palette_buttons => @default-return gtk::Inhibit(false), move |_, active| {
            self_.set_accent();
            gtk::Inhibit(false)
        }));
        let is_dark = style_manager.is_dark();
        imp.css_provider.set(provider).unwrap();
        imp.accent_button.set(accent_color_button).unwrap();
        imp.color_editor.set(inner).unwrap();
        imp.palette_buttons.replace(vec![]);
        imp.palette_box.set(palette_box).unwrap();
        imp.style_manager.set(style_manager).unwrap();
        imp.use_palette_switch.set(use_palette_switch).unwrap();
        // set widget state

        self_.set_palette_buttons(is_dark);
        self_.connect_color_button();

        self_
    }

    fn connect_color_button(&self) {
        let imp = imp::AccentEditor::from_instance(&self);

        imp.accent_button.get().unwrap().connect_rgba_notify(
            glib::clone!(@weak self as self_, => move |_| {
                self_.set_accent();
            }),
        );
    }

    fn set_accent(&self) {
        let imp = imp::AccentEditor::from_instance(&self);
        let palette_buttons = imp.palette_buttons.borrow();
        let palette_buttons = palette_buttons.as_slice();
        let use_palette_switch = &imp.use_palette_switch.get().unwrap();
        let style_manager = imp.style_manager.get().unwrap();
        let css_provider = imp.css_provider.get().unwrap();
        let accent_button = imp.accent_button.get().unwrap();

        let c = accent_button.rgba();
        let mut lch_c = util::get_lch(c);
        let is_dark = style_manager.is_dark();

        if use_palette_switch.is_active() {
            // find matching color from palette
            let palette = if is_dark {
                accent_palette::get_dark_palette(&self.style_context())
            } else {
                accent_palette::get_light_palette(&self.style_context())
            };

            let (i, _) =
                palette
                    .iter()
                    .enumerate()
                    .fold((0, f32::MAX), |(pre_i, pre_d), (i, cur_p)| {
                        let c_comp: Lch = SRGB::from(cur_p.accent_color_bg).0.into_color();
                        // TODO better distance calculation accounting for wrapping and with normalized values
                        let dh = f32::min(
                            c_comp.hue.to_degrees() - lch_c.hue.to_degrees(),
                            360.0 - c_comp.hue.to_degrees() - lch_c.hue.to_degrees(),
                        )
                        .abs();
                        let dc = (c_comp.chroma - lch_c.chroma).abs();
                        let cur_d = if c_comp.chroma < 64.0 {
                            (dc.powi(2) + dh.powi(2))
                        } else {
                            (dc + dh.powi(2))
                        };
                        if pre_d < cur_d {
                            (pre_i, pre_d)
                        } else {
                            (i, cur_d)
                        }
                    });
            let b = &palette_buttons[i];
            b.set_active(true);
            let p = &palette[i];
            let accent_as_fg: SRGB = SRGB::from(p.accent_color);
            let accent_bg: SRGB = SRGB::from(p.accent_color);
            let fg = SRGB::from(p.accent_color_fg);
            let mut style = css_provider.to_str().to_string();
            style += &format!(
                r#"
            @define-color accent_color #{};
            @define-color accent_bg_color #{};
            @define-color accent_fg_color #{};
            "#,
                util::hex_from_rgba(accent_as_fg.into()),
                util::hex_from_rgba(accent_bg.into()),
                util::hex_from_rgba(fg.into())
            );
            css_provider.load_from_data(style.as_bytes());
        } else {
            // derive colors automatically

            // TODO get lightness from view_bg, window_bg, or headerbar_bg
            // use the worst case for base lighness & contrast when deriving colors
            (lch_c.l) = if is_dark {
                Lch::<D65>::min_l()
            } else {
                Lch::<D65>::max_l()
            };
            let (derived_fg, fg_contrast, bg_contrast) = if is_dark {
                (SRGB(Srgb::new(1.0, 1.0, 1.0)), 15.0, 5.0)
            } else {
                (SRGB(Srgb::new(0.0, 0.0, 0.0)), 7.0, 1.1) // 7.0 & 1.1 are minimum required
            };
            let derived_accent_as_fg: SRGB =
                if let Ok(fg_c) = util::derive_color(lch_c, Some(fg_contrast), None) {
                    SRGB(fg_c.into_color())
                } else {
                    derived_fg
                };
            let derived_bg: SRGB = SRGB(
                util::derive_color(lch_c, Some(bg_contrast), None)
                    .unwrap_or_else(|e| {
                        dbg!(e);
                        lch_c
                    })
                    .into_color(),
            );

            let mut style = css_provider.to_str().to_string();
            style += &format!(
                r#"
            @define-color accent_color #{};
            @define-color accent_bg_color #{};
            @define-color accent_fg_color #{};
            "#,
                util::hex_from_rgba(derived_accent_as_fg.into()),
                util::hex_from_rgba(derived_bg.into()),
                util::hex_from_rgba(derived_fg.into())
            );
            css_provider.load_from_data(style.as_bytes());
        }
    }

    fn set_palette_buttons(&self, is_dark: bool) {
        let imp = imp::AccentEditor::from_instance(&self);
        let palette_box = imp.palette_box.get().unwrap();
        let mut palette_toggles = vec![];

        while let Some(c) = palette_box.first_child() {
            palette_box.remove(&c);
        }

        let palette = if is_dark {
            accent_palette::get_dark_palette(&self.style_context())
        } else {
            accent_palette::get_light_palette(&self.style_context())
        };

        for c in palette {
            view! {
                button = &ToggleButton {
                    add_css_class: "opaque",
                    set_widget_name: &c.name,
                    set_group: palette_toggles.get(0),
                    connect_toggled: glib::clone!(@weak self as self_, @weak imp.accent_button as accent_button => move |_| {
                        accent_button.get().unwrap().set_rgba(&c.accent_color_bg);
                        self_.set_accent();
                    })
                }
            };
            palette_box.append(&button);
            palette_toggles.push(button);
        }

        imp.palette_buttons.replace(palette_toggles);
    }
}
