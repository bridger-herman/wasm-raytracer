//! Represents a pixel
// Original by Wagner Correa, 1999
// Turned to C++ by Robert Osada, 2000
// Updateded by Stephen J. Guy, 2017
// Translated to Rust by Bridger Herman, 2018

use std::ops::{Add, Mul, Sub};

// use rand;
// use rand::Rng;

fn clamp_pix_component(value: f64) -> f64 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

fn lerp_pix_component(low: f64, high: f64, amount: f64) -> f64 {
    clamp_pix_component((high - low) * amount + low)
}

fn raw_to_pix_component(value: u8) -> f64 {
    f64::from(value) / f64::from(u8::max_value())
}

fn pix_component_to_raw(value: f64) -> u8 {
    (clamp_pix_component(value) * f64::from(u8::max_value())) as u8
}

/// Struct for representing pixels on a byte level (for image representation)
#[derive(Debug, Copy, Clone)]
pub struct RawPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Pixel> for RawPixel {
    fn from(pix: Pixel) -> RawPixel {
        Self {
            r: pix_component_to_raw(pix.r),
            g: pix_component_to_raw(pix.g),
            b: pix_component_to_raw(pix.b),
            a: pix_component_to_raw(pix.a),
        }
    }
}

impl From<RawPixel> for Pixel {
    fn from(raw: RawPixel) -> Pixel {
        Self {
            r: raw_to_pix_component(raw.r),
            g: raw_to_pix_component(raw.g),
            b: raw_to_pix_component(raw.b),
            a: raw_to_pix_component(raw.a),
            with_clamping: true,
        }
    }
}

impl<'a> From<&'a RawPixel> for Vec<u8> {
    fn from(raw: &'a RawPixel) -> Vec<u8> {
        vec![raw.r, raw.g, raw.b, raw.a]
    }
}

/// Holds all pixel information as a float
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
    pub with_clamping: bool,
}

impl Pixel {
    pub fn from_slice_unclamped(numbers: &[f64]) -> Self {
        Self {
            r: numbers[0],
            g: numbers[1],
            b: numbers[2],
            a: 1.0,
            with_clamping: false,
        }
    }

    pub fn from_pix_unclamped(mut pix: Self) -> Self {
        pix.with_clamping = false;
        pix
    }

    /// Construct a new pixel from R, G, B, A values, but don't clamp values
    pub fn from_rgba_unclamped(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            r,
            g,
            b,
            a,
            with_clamping: false,
        }
    }

    /// Construct a new pixel from R, G, B, A values
    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            r,
            g,
            b,
            a,
            with_clamping: true,
        }
    }

    /// Construct a new pixel from R, G, B values
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    // /// Contruct a random pixel, but only RGB values
    // pub fn random_rgb() -> Self {
    // let mut rng = rand::thread_rng();
    // Self {
    // r: rng.gen::<f64>(),
    // g: rng.gen::<f64>(),
    // b: rng.gen::<f64>(),
    // a: 1.0,
    // with_clamping: true,
    // }
    // }

    // /// Construct a random pixel
    // pub fn random() -> Self {
    // let mut rng = rand::thread_rng();
    // Self {
    // r: rng.gen::<f64>(),
    // g: rng.gen::<f64>(),
    // b: rng.gen::<f64>(),
    // a: rng.gen::<f64>(),
    // with_clamping: true,
    // }
    // }

    /// Construct a new pixel with all values clamped, consuming self
    pub fn clamp(self) -> Self {
        Self {
            r: clamp_pix_component(self.r),
            g: clamp_pix_component(self.g),
            b: clamp_pix_component(self.b),
            a: clamp_pix_component(self.a),
            with_clamping: self.with_clamping,
        }
    }

    /// Set RGBA components
    pub fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
        if self.with_clamping {
            self.clamp();
        }
    }

    /// Set RGB components
    pub fn set_rgb(&mut self, r: f64, g: f64, b: f64) {
        self.r = r;
        self.g = g;
        self.b = b;
        if self.with_clamping {
            self.clamp();
        }
    }

    /// Luminance of a pixel
    /// Luminance formula from Wikipedia
    pub fn luminance(&self) -> f64 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    /// Linearly interpolate with another pixel
    pub fn lerp(&self, other: &Pixel, amount: f64) -> Pixel {
        Self {
            r: lerp_pix_component(self.r, other.r, amount),
            g: lerp_pix_component(self.g, other.g, amount),
            b: lerp_pix_component(self.b, other.b, amount),
            a: lerp_pix_component(self.a, other.a, amount),
            with_clamping: self.with_clamping,
        }
    }

    /// Quantize the bits of this pixel into bins
    pub fn quant(&self, num_bins: usize) -> Pixel {
        let raw = RawPixel::from(*self);
        let shift = 8 - num_bins;
        let multiplier = 255 / (255 >> shift);
        let new_r = raw.r >> shift;
        let new_g = raw.g >> shift;
        let new_b = raw.b >> shift;

        let raw_quant = RawPixel {
            r: new_r * multiplier,
            g: new_g * multiplier,
            b: new_b * multiplier,
            a: raw.a,
        };
        Pixel::from(raw_quant)
    }
}

impl<'a> From<&'a [f64]> for Pixel {
    fn from(numbers: &'a [f64]) -> Self {
        Self {
            r: numbers[0],
            g: numbers[1],
            b: numbers[2],
            a: 1.0,
            with_clamping: true,
        }
    }
}

impl Add for Pixel {
    type Output = Self;
    // Add two pixels, consuming self
    fn add(self, other: Self) -> Self {
        let ret = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: 1.0,
            with_clamping: self.with_clamping,
        };
        if self.with_clamping {
            ret.clamp()
        } else {
            ret
        }
    }
}

impl Sub for Pixel {
    type Output = Self;
    // Add two pixels, consuming self
    fn sub(self, other: Self) -> Self {
        let ret = Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: 1.0,
            with_clamping: self.with_clamping,
        };
        if self.with_clamping {
            ret.clamp()
        } else {
            ret
        }
    }
}

impl Mul for Pixel {
    type Output = Self;
    // Multiply two pixels, consuming self
    fn mul(self, other: Self) -> Self {
        let ret = Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: 1.0,
            with_clamping: self.with_clamping,
        };
        if self.with_clamping {
            ret.clamp()
        } else {
            ret
        }
    }
}

impl Mul<f64> for Pixel {
    type Output = Self;
    // Multiply two pixels, consuming self
    fn mul(self, other: f64) -> Self {
        let ret = Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: 1.0,
            with_clamping: self.with_clamping,
        };
        if self.with_clamping {
            ret.clamp()
        } else {
            ret
        }
    }
}
