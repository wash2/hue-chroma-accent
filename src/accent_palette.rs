use adw::{
    gtk::{gdk::RGBA, StyleContext},
    prelude::StyleContextExt,
};
// TODO make palette editable

#[derive(Debug, Clone)]
pub struct AccentPalette {
    pub name: String,
    pub accent_color: RGBA,
    pub accent_color_bg: RGBA,
    pub accent_color_fg: RGBA,
}

pub fn get_light_palette(sc: &StyleContext) -> Vec<AccentPalette> {
    let red_light: AccentPalette = AccentPalette {
        name: "RED_LIGHT".into(),

        accent_color: sc.lookup_color("red_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("red_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("red_light_fg").unwrap(),
    };

    let orange_light: AccentPalette = AccentPalette {
        name: "ORANGE_LIGHT".into(),

        accent_color: sc.lookup_color("orange_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("orange_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("orange_light_fg").unwrap(),
    };

    let yellow_light: AccentPalette = AccentPalette {
        name: "YELLOW_LIGHT".into(),

        accent_color: sc.lookup_color("yellow_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("yellow_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("yellow_light_fg").unwrap(),
    };

    let green_light: AccentPalette = AccentPalette {
        name: "GREEN_LIGHT".into(),

        accent_color: sc.lookup_color("green_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("green_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("green_light_fg").unwrap(),
    };

    let blue_light: AccentPalette = AccentPalette {
        name: "BLUE_LIGHT".into(),

        accent_color: sc.lookup_color("blue_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("blue_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("blue_light_fg").unwrap(),
    };

    let purple_light: AccentPalette = AccentPalette {
        name: "PURPLE_LIGHT".into(),

        accent_color: sc.lookup_color("purple_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("purple_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("purple_light_fg").unwrap(),
    };

    let brown_light: AccentPalette = AccentPalette {
        name: "BROWN_LIGHT".into(),

        accent_color: sc.lookup_color("brown_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("brown_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("brown_light_fg").unwrap(),
    };

    let grey_light: AccentPalette = AccentPalette {
        name: "GREY_LIGHT".into(),

        accent_color: sc.lookup_color("grey_light_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("grey_light_bg").unwrap(),
        accent_color_fg: sc.lookup_color("grey_light_fg").unwrap(),
    };

    vec![
        red_light,
        orange_light,
        yellow_light,
        green_light,
        blue_light,
        purple_light,
        brown_light,
        grey_light,
    ]
}

pub fn get_dark_palette(sc: &StyleContext) -> Vec<AccentPalette> {
    let red_dark: AccentPalette = AccentPalette {
        name: "RED_DARK".into(),

        accent_color: sc.lookup_color("red_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("red_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("red_dark_fg").unwrap(),
    };

    let orange_dark: AccentPalette = AccentPalette {
        name: "ORANGE_DARK".into(),

        accent_color: sc.lookup_color("orange_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("orange_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("orange_dark_fg").unwrap(),
    };

    let yellow_dark: AccentPalette = AccentPalette {
        name: "YELLOW_DARK".into(),

        accent_color: sc.lookup_color("yellow_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("yellow_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("yellow_dark_fg").unwrap(),
    };

    let green_dark: AccentPalette = AccentPalette {
        name: "GREEN_DARK".into(),

        accent_color: sc.lookup_color("green_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("green_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("green_dark_fg").unwrap(),
    };

    let blue_dark: AccentPalette = AccentPalette {
        name: "BLUE_DARK".into(),

        accent_color: sc.lookup_color("blue_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("blue_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("blue_dark_fg").unwrap(),
    };

    let purple_dark: AccentPalette = AccentPalette {
        name: "PURPLE_DARK".into(),

        accent_color: sc.lookup_color("purple_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("purple_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("purple_dark_fg").unwrap(),
    };

    let brown_dark: AccentPalette = AccentPalette {
        name: "BROWN_DARK".into(),

        accent_color: sc.lookup_color("brown_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("brown_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("brown_dark_fg").unwrap(),
    };

    let grey_dark: AccentPalette = AccentPalette {
        name: "GREY_DARK".into(),

        accent_color: sc.lookup_color("grey_dark_as_fg").unwrap(),
        accent_color_bg: sc.lookup_color("grey_dark_bg").unwrap(),
        accent_color_fg: sc.lookup_color("grey_dark_fg").unwrap(),
    };

    vec![
        red_dark,
        orange_dark,
        yellow_dark,
        green_dark,
        blue_dark,
        purple_dark,
        brown_dark,
        grey_dark,
    ]
}
