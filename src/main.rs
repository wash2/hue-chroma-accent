// SPDX-License-Identifier: MPL-2.0-only

use adw::gtk::{gdk::Display, gio, glib, prelude::*, CssProvider, StyleContext};
use adw::{gtk, ActionRow, Application, ApplicationWindow, HeaderBar};
use components::accent_editor::AccentEditor;
use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR, PROFILE, VERSION};
use gettextrs::{gettext, LocaleCategory};

mod accent_palette;
mod components;
mod config;
mod util;

fn setup_shortcuts(app: &Application) {
    //quit shortcut
    app.set_accels_for_action("win.quit", &["<primary>W", "Escape"]);
}

fn load_css() -> CssProvider {
    let provider = CssProvider::new();
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
    provider.load_from_resource("/com/github/wash2/HueChromaAccent/style.css");
    provider
}

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Hue Chroma Accent"));

    let app = Application::builder().application_id(APP_ID).build();
    gio::resources_register_include!("compiled.gresource").unwrap();

    app.connect_startup(|app| {
        setup_shortcuts(app);
    });
    app.connect_activate(move |app| {
        let provider = load_css();
        let accent_editor = AccentEditor::new(provider);

        let window = ApplicationWindow::builder()
            .application(app)
            .title(APP_ID)
            .default_width(350)
            // add content to window
            .content(&accent_editor)
            .icon_name(APP_ID)
            .build();

        window.show();
    });
    app.run();
}
