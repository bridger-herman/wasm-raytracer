//! Represents a collection of pixels (an image)
// Original by Wagner Correa, 1999
// Turned to C++ by Robert Osada, 2000
// Updateded by Stephen J. Guy, 2017
// Translated to Rust by Bridger Herman, 2018

use ext_image;
use ext_image::{ColorType, GenericImage};

use pixel::{Pixel, RawPixel};

/// A struct representing a collection of pixels
#[derive(Debug, Clone)]
pub struct Image {
    pixels: Vec<RawPixel>,

    pub width: usize,
    pub height: usize,
    // sampling_method:
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![
                RawPixel {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0
                };
                width * height
            ],
            width,
            height,
        }
    }

    /// Load an image from a file, using the `image` crate
    pub fn from_file(filename: &str) -> Self {
        let external_image =
            ext_image::open(filename).expect("Unable to open image");

        let (width32, height32) = external_image.dimensions();
        let (width, height) = (width32 as usize, height32 as usize);

        let pixel_bytes = external_image.raw_pixels();

        let mut final_pixels = vec![
            RawPixel {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            };
            width * height
        ];

        // Iterate through based on how many channels there are
        let step = match external_image.color() {
            ColorType::Gray(_) => 1,
            ColorType::Palette(_) => 1, // Unknown if this is correct
            ColorType::GrayA(_) => 2,
            ColorType::RGB(_) => 3,
            ColorType::RGBA(_) => 4,
        };

        for (final_index, byte_index) in
            (0..pixel_bytes.len()).step_by(step).enumerate()
        {
            final_pixels[final_index] = match step {
                1 => RawPixel {
                    r: pixel_bytes[byte_index],
                    g: pixel_bytes[byte_index],
                    b: pixel_bytes[byte_index],
                    a: u8::max_value(),
                },
                2 => RawPixel {
                    r: pixel_bytes[byte_index],
                    g: pixel_bytes[byte_index],
                    b: pixel_bytes[byte_index],
                    a: pixel_bytes[byte_index + 1],
                },
                3 => RawPixel {
                    r: pixel_bytes[byte_index],
                    g: pixel_bytes[byte_index + 1],
                    b: pixel_bytes[byte_index + 2],
                    a: u8::max_value(),
                },
                4 => RawPixel {
                    r: pixel_bytes[byte_index],
                    g: pixel_bytes[byte_index + 1],
                    b: pixel_bytes[byte_index + 2],
                    a: pixel_bytes[byte_index + 3],
                },
                _ => RawPixel {
                    r: u8::min_value(),
                    g: u8::min_value(),
                    b: u8::min_value(),
                    a: u8::max_value(),
                },
            };
        }

        Self {
            pixels: final_pixels,
            width,
            height,
        }
    }

    /// Add a background to an image (overwrites image)
    pub fn with_background(mut self, color: Pixel) -> Image {
        self.pixels = vec![RawPixel::from(color); self.pixels.len()];
        self
    }

    /// Check to see if a coordinate is valid and inside an image
    pub fn is_valid_coord(&self, row: usize, col: usize) -> bool {
        col < self.width && row < self.height
    }

    pub fn get_pixel(&self, row: usize, col: usize) -> Option<Pixel> {
        if !self.is_valid_coord(row, col) {
            return None;
        }
        Some(Pixel::from(self.pixels[row * self.width + col]))
    }

    pub fn get_pixel_mirrored(&self, row: f64, col: f64) -> Pixel {
        let (mut row, mut col) = (row.abs(), col.abs());

        // Reflect the image if it's over boundaries on the high-indexed
        // side
        if row >= self.height as f64 {
            row = self.height as f64 - (row - self.height as f64) - 1.0;
        }
        if col >= self.width as f64 {
            col = self.width as f64 - (col - self.width as f64) - 1.0;
        }

        let (row, col) =
            (row.abs().round() as usize, col.abs().round() as usize);

        Pixel::from(self.pixels[row * self.width + col])
    }

    pub fn get_pixels(&self) -> Vec<Pixel> {
        self.pixels.iter().map(|&raw| Pixel::from(raw)).collect()
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, pix: Pixel) {
        if !self.is_valid_coord(row, col) {
            return;
        }
        self.pixels[row * self.width + col] = RawPixel::from(pix)
    }

    /// Write the image to a file
    /// Supported formats:
    /// - PNG
    /// - JPG
    /// - BMP
    /// - GIF
    /// - ICO
    /// - TIFF
    /// - Webp
    /// - PNM (PPM)
    pub fn write(&self, filename: &str) -> Result<(), String> {
        let use_alpha = match &filename[filename.len() - 4..] {
            ".png" => true,
            _ => false,
        };

        let mut pixel_bytes = if use_alpha {
            vec![0; self.width * self.height * 4]
        } else {
            vec![0; self.width * self.height * 3]
        };
        let mut byte_index = 0;

        // Convert back to bytes
        for pix in &self.pixels {
            let RawPixel { r, g, b, a } = pix;
            pixel_bytes[byte_index] = *r;
            byte_index += 1;
            pixel_bytes[byte_index] = *g;
            byte_index += 1;
            pixel_bytes[byte_index] = *b;
            byte_index += 1;
            if use_alpha {
                pixel_bytes[byte_index] = *a;
                byte_index += 1;
            }
        }

        if use_alpha {
            ext_image::save_buffer(
                filename,
                &pixel_bytes,
                self.width as u32,
                self.height as u32,
                ColorType::RGBA(8),
            ).map_err(|e| e.to_string())?;
        } else {
            ext_image::save_buffer(
                filename,
                &pixel_bytes,
                self.width as u32,
                self.height as u32,
                ColorType::RGB(8),
            ).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
