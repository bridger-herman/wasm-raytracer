//! Represents a collection of pixels (an image)
// Original by Wagner Correa, 1999
// Turned to C++ by Robert Osada, 2000
// Updateded by Stephen J. Guy, 2017
// Translated to Rust by Bridger Herman, 2018

use std::io::BufWriter;

use png::HasParameters;

use crate::pixel::{Pixel, RawPixel};

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

    pub fn to_bytes(&self) -> Vec<u8> {
        self.pixels.iter().map(Vec::<u8>::from).flatten().collect()
    }

    pub fn to_png_bytes(&self) -> Vec<u8> {
        let file = Vec::new();
        let mut w = BufWriter::new(file);

        {
            let mut encoder = png::Encoder::new(
                &mut w,
                self.width as u32,
                self.height as u32,
            );
            encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

            let mut writer = encoder
                .write_header()
                .expect("Unable to write image header");

            let raw_bytes = self.to_bytes();
            writer
                .write_image_data(&raw_bytes)
                .expect("Unable to write image");
        }

        w.into_inner().expect("Unable to get png byte vector")
    }
}
