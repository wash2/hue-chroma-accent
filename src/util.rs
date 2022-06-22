// SPDX-License-Identifier: MPL-2.0-only

use adw::gtk::gdk::RGBA;
use float_cmp::approx_eq;
use hex::encode;
// use kmeans_colors::{get_kmeans_hamerly, Kmeans, Sort};
use palette::{Clamp, IntoColor, Lch, Pixel, RelativeContrast, Srgb};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SRGB(pub Srgb);

pub fn hex_from_rgba(rgba: RGBA) -> String {
    let c = SRGB::from(rgba);
    let hex = encode::<[u8; 3]>(Srgb::into_raw(c.0.into_format()));
    format!("{hex}")
}

impl SRGB {
    pub fn into_inner(self) -> Srgb {
        self.0
    }
}
impl From<Srgb> for SRGB {
    fn from(c: Srgb) -> Self {
        Self(c)
    }
}
impl From<RGBA> for SRGB {
    fn from(rgba: RGBA) -> Self {
        Self(Srgb::new(rgba.red(), rgba.green(), rgba.blue()))
    }
}

impl Into<RGBA> for SRGB {
    fn into(self) -> RGBA {
        RGBA::new(self.0.red, self.0.green, self.0.blue, 1.0)
    }
}

impl Into<Srgb> for SRGB {
    fn into(self) -> Srgb {
        self.0
    }
}

pub fn get_lch(c: RGBA) -> Lch {
    let c: Srgb = SRGB::from(c).into();
    c.into_format().into_color()
}

pub fn derive_color(
    mut lch_color: Lch,
    contrast: Option<f32>,
    lighten: Option<bool>,
) -> anyhow::Result<Lch> {
    // lighten or darken
    // TODO closed form solution using Lch color space contrast formula?
    // for now do binary search...

    if let Some(contrast) = contrast {
        let (min, max) = match lighten {
            Some(b) if b => (lch_color.l, 100.0),
            Some(_) => (0.0, lch_color.l),
            None => (0.0, 100.0),
        };
        let (mut l, mut r) = (min, max);

        for _ in 0..100 {
            dbg!((l, r));
            let cur_guess_lightness = (l + r) / 2.0;
            let mut cur_guess = lch_color;
            cur_guess.l = cur_guess_lightness;
            let cur_contrast = lch_color.get_contrast_ratio(&cur_guess.into_color());
            let contrast_dir = contrast > cur_contrast;
            let lightness_dir = lch_color.l < cur_guess.l;
            if approx_eq!(f32, contrast, cur_contrast, ulps = 4) {
                lch_color = cur_guess;
                break;
                // TODO fix
            } else if lightness_dir && contrast_dir || !lightness_dir && !contrast_dir {
                l = cur_guess_lightness;
            } else {
                r = cur_guess_lightness;
            }
        }

        // clamp to valid value in range
        lch_color.clamp_self();

        // verify contrast
        let actual_contrast = lch_color.get_contrast_ratio(&lch_color.into_color());
        if !approx_eq!(f32, contrast, actual_contrast, ulps = 4) {
            anyhow::bail!("Failed to derive color with contrast {}", contrast,);
        }

        Ok(lch_color.into_color())
    } else {
        // maximize contrast if no constraint is given
        if lch_color.l > 50.0 {
            Ok(palette::named::BLACK.into_format().into_color())
        } else {
            Ok(palette::named::WHITE.into_format().into_color())
        }
    }
}
