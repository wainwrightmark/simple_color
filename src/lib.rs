use glam::prelude::{Vec3, Vec4};
use thiserror::Error;

/// Non-linear standard RGB with alpha.
///
/// SDR colors are in `[0.0, 1.0]`. Values above `1.0` are HDR intensities.
/// </div>
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]

pub struct Srgba {
    /// The red channel. [0.0, 1.0] for SDR colors.
    pub red: f32,
    /// The green channel. [0.0, 1.0] for SDR colors.
    pub green: f32,
    /// The blue channel. [0.0, 1.0] for SDR colors.
    pub blue: f32,
    /// The alpha channel. [0.0, 1.0]
    pub alpha: f32,
}

impl Srgba {
    // The standard VGA colors, with alpha set to 1.0.
    // https://en.wikipedia.org/wiki/Web_colors#Basic_colors

    /// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully black color with full alpha.
    pub const BLACK: Srgba = Srgba::new(0.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgba(0%, 0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully transparent color with no alpha (alpha = 0.0).
    #[doc(alias = "transparent")]
    pub const NONE: Srgba = Srgba::new(0.0, 0.0, 0.0, 0.0);
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully white color with full alpha.
    pub const WHITE: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully red color with full alpha.
    pub const RED: Srgba = Srgba::new(1.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully green color with full alpha.
    pub const GREEN: Srgba = Srgba::new(0.0, 1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div><br />
    /// A fully blue color with full alpha.
    pub const BLUE: Srgba = Srgba::new(0.0, 0.0, 1.0, 1.0);

    /// Construct a new [`Srgba`] color from components.
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0.0, 1.0]
    /// * `green` - Green channel. [0.0, 1.0]
    /// * `blue` - Blue channel. [0.0, 1.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Construct a new [`Srgba`] color from (r, g, b) components, with the default alpha (1.0).
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0.0, 1.0]
    /// * `green` - Green channel. [0.0, 1.0]
    /// * `blue` - Blue channel. [0.0, 1.0]
    pub const fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 1.0,
        }
    }

    /// Return a copy of this color with the red channel set to the given value.
    pub const fn with_red(self, red: f32) -> Self {
        Self { red, ..self }
    }

    /// Return a copy of this color with the green channel set to the given value.
    pub const fn with_green(self, green: f32) -> Self {
        Self { green, ..self }
    }

    /// Return a copy of this color with the blue channel set to the given value.
    pub const fn with_blue(self, blue: f32) -> Self {
        Self { blue, ..self }
    }

    /// New `Srgba` from a CSS-style hexadecimal string.    
    /// ```
    pub fn hex<T: AsRef<str>>(hex: T) -> Result<Self, HexColorError> {
        let hex = hex.as_ref();
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match hex.len() {
            // RGB
            3 => {
                let [l, b] = u16::from_str_radix(hex, 16)?.to_be_bytes();
                let (r, g, b) = (l & 0x0F, (b & 0xF0) >> 4, b & 0x0F);
                Ok(Self::rgb_u8((r << 4) | r, (g << 4) | g, (b << 4) | b))
            }
            // RGBA
            4 => {
                let [l, b] = u16::from_str_radix(hex, 16)?.to_be_bytes();
                let (r, g, b, a) = ((l & 0xF0) >> 4, l & 0xF, (b & 0xF0) >> 4, b & 0x0F);
                Ok(Self::rgba_u8(
                    (r << 4) | r,
                    (g << 4) | g,
                    (b << 4) | b,
                    (a << 4) | a,
                ))
            }
            // RRGGBB
            6 => {
                let [_, r, g, b] = u32::from_str_radix(hex, 16)?.to_be_bytes();
                Ok(Self::rgb_u8(r, g, b))
            }
            // RRGGBBAA
            8 => {
                let [r, g, b, a] = u32::from_str_radix(hex, 16)?.to_be_bytes();
                Ok(Self::rgba_u8(r, g, b, a))
            }
            _ => Err(HexColorError::Length),
        }
    }

    /// Convert this color to CSS-style hexadecimal notation.    
    pub fn to_hex(&self) -> String {
        let [r, g, b, a] = self.to_u8_array();
        match a {
            255 => format!("#{r:02X}{g:02X}{b:02X}"),
            _ => format!("#{r:02X}{g:02X}{b:02X}{a:02X}"),
        }
    }

    /// New `Srgba` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0, 255]
    /// * `g` - Green channel. [0, 255]
    /// * `b` - Blue channel. [0, 255]
    ///
    /// See also [`Srgba::new`], [`Srgba::rgba_u8`], [`Srgba::hex`].
    pub fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::from_u8_array_no_alpha([r, g, b])
    }

    // Float operations in const fn are not stable yet
    // see https://github.com/rust-lang/rust/issues/57241
    /// New `Srgba` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0, 255]
    /// * `g` - Green channel. [0, 255]
    /// * `b` - Blue channel. [0, 255]
    /// * `a` - Alpha channel. [0, 255]
    ///
    /// See also [`Srgba::new`], [`Srgba::rgb_u8`], [`Srgba::hex`].
    pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_u8_array([r, g, b, a])
    }

    /// Converts a non-linear sRGB value to a linear one via [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).
    pub fn gamma_function(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }
        if value <= 0.04045 {
            value / 12.92 // linear falloff in dark values
        } else {
            f32::powf((value + 0.055) / 1.055, 2.4) // gamma curve in other area
        }
    }

    /// Converts a linear sRGB value to a non-linear one via [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).
    pub fn gamma_function_inverse(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }

        if value <= 0.0031308 {
            value * 12.92 // linear falloff in dark values
        } else {
            (1.055 * f32::powf(value, 1.0 / 2.4)) - 0.055 // gamma curve in other area
        }
    }
}

impl Srgba {
    pub const fn rgb_hex(mut number: u32) -> Srgba {
        let b = number % 256;
        number /= 256;
        let g = number % 256;
        number /= 256;
        let r = number % 256;

        let blue = b as f32 / 256.0;
        let green = g as f32 / 256.0;
        let red = r as f32 / 256.0;

        Srgba {
            alpha: 1.0,
            blue,
            green,
            red,
        }
    }

    pub const fn rgba_hex(mut number: u32) -> Srgba {
        let a = number % 256;
        number /= 256;
        let b = number % 256;
        number /= 256;
        let g = number % 256;
        number /= 256;
        let r = number % 256;

        let alpha = a as f32 / 256.0;
        let blue = b as f32 / 256.0;
        let green = g as f32 / 256.0;
        let red = r as f32 / 256.0;

        Srgba {
            alpha,
            blue,
            green,
            red,
        }
    }
}

impl Default for Srgba {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Srgba {
    #[inline]
    pub fn mix(&self, other: &Self, factor: f32) -> Self {
        let n_factor = 1.0 - factor;
        Self {
            red: self.red * n_factor + other.red * factor,
            green: self.green * n_factor + other.green * factor,
            blue: self.blue * n_factor + other.blue * factor,
            alpha: self.alpha * n_factor + other.alpha * factor,
        }
    }

    #[inline]
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self { alpha, ..*self }
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    #[inline]
    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    #[inline]
    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dr = self.red - other.red;
        let dg = self.green - other.green;
        let db = self.blue - other.blue;
        dr * dr + dg * dg + db * db
    }

    pub fn to_f32_array(self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    pub fn to_f32_array_no_alpha(self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    pub fn from_f32_array(color: [f32; 4]) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: color[3],
        }
    }

    pub fn from_f32_array_no_alpha(color: [f32; 3]) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: 1.0,
        }
    }

    pub fn to_vec4(self) -> Vec4 {
        Vec4::new(self.red, self.green, self.blue, self.alpha)
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.red, self.green, self.blue)
    }

    pub fn from_vec4(color: Vec4) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: color[3],
        }
    }

    pub fn from_vec3(color: Vec3) -> Self {
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha: 1.0,
        }
    }

    #[allow(unused)]
    fn to_u8_array(self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
            .map(|v| f32::round(v.clamp(0.0, 1.0) * 255.0) as u8)
    }

    #[allow(unused)]
    fn to_u8_array_no_alpha(self) -> [u8; 3] {
        [self.red, self.green, self.blue].map(|v| f32::round(v.clamp(0.0, 1.0) * 255.0) as u8)
    }

    fn from_u8_array(color: [u8; 4]) -> Self {
        Self::from_f32_array(color.map(|u| u as f32 / 255.0))
    }

    fn from_u8_array_no_alpha(color: [u8; 3]) -> Self {
        Self::from_f32_array_no_alpha(color.map(|u| u as f32 / 255.0))
    }
}

/// Error returned if a hex string could not be parsed as a color.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HexColorError {
    /// Parsing error.
    #[error("Invalid hex string")]
    Parse(#[from] core::num::ParseIntError),
    /// Invalid length.
    #[error("Unexpected length of hex string")]
    Length,
    /// Invalid character.
    #[error("Invalid hex char")]
    Char(char),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn euclidean_distance() {
        // White to black
        let a = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let b = Srgba::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a.distance_squared(&b), 3.0);

        // Alpha shouldn't matter
        let a = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let b = Srgba::new(1.0, 1.0, 1.0, 0.0);
        assert_eq!(a.distance_squared(&b), 3.0);

        // Red to green
        let a = Srgba::new(0.0, 0.0, 0.0, 1.0);
        let b = Srgba::new(1.0, 0.0, 0.0, 1.0);
        assert_eq!(a.distance_squared(&b), 1.0);
    }

    #[test]
    fn hex_color() {
        assert_eq!(Srgba::hex("FFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("FFFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("FFFFFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("FFFFFFFF"), Ok(Srgba::WHITE));
        assert_eq!(Srgba::hex("000"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("000F"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("000000"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("000000FF"), Ok(Srgba::BLACK));
        assert_eq!(Srgba::hex("03a9f4"), Ok(Srgba::rgb_u8(3, 169, 244)));
        assert_eq!(Srgba::hex("yy"), Err(HexColorError::Length));
        assert_eq!(Srgba::hex("#f2a"), Ok(Srgba::rgb_u8(255, 34, 170)));
        assert_eq!(Srgba::hex("#e23030"), Ok(Srgba::rgb_u8(226, 48, 48)));
        assert_eq!(Srgba::hex("#ff"), Err(HexColorError::Length));
        assert_eq!(Srgba::hex("11223344"), Ok(Srgba::rgba_u8(17, 34, 51, 68)));
        assert_eq!(Srgba::hex("1234"), Ok(Srgba::rgba_u8(17, 34, 51, 68)));
        assert_eq!(Srgba::hex("12345678"), Ok(Srgba::rgba_u8(18, 52, 86, 120)));
        assert_eq!(Srgba::hex("4321"), Ok(Srgba::rgba_u8(68, 51, 34, 17)));

        assert!(matches!(Srgba::hex("yyy"), Err(HexColorError::Parse(_))));
        assert!(matches!(Srgba::hex("##fff"), Err(HexColorError::Parse(_))));
    }
}
