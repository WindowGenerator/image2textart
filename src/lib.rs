use colored::Colorize;
use image::{imageops, DynamicImage, GenericImageView, Rgba};
use std::fmt;
use std::process::exit;
use terminal_size::{terminal_size, Height, Width};

#[cfg(any(target_os = "macos", target_os = "linux"))]
const CHAR_WIDTH: f32 = 0.5;

#[cfg(target_os = "windows")]
const CHAR_WIDTH: f32 = 0.714;

#[derive(Debug)]
pub enum Color {
    NoColor = 0,
    Gray,
    Char,
    Background,
    Full,
}

pub struct Options {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub charset: String,
    pub color: Color,
    pub ratio: f32,
    pub invert: bool,
}

pub type Coordinate = (u32, u32);
pub type RGB = (u8, u8, u8);
pub type Saturation = f64;
pub type Char = char;

#[derive(Debug)]
pub struct Pixel(Coordinate, Char, RGB, Saturation);

pub struct TextArtMatrix {
    pixels: Vec<Pixel>,
    color: Color,
}

impl fmt::Display for TextArtMatrix {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row_ptr = 0;

        let rended_textart: String = self
            .pixels
            .iter()
            .map(|pixel| {
                let Pixel((_, ay), symbol, (r, g, b), _) = pixel;
                let mut current_char = symbol.to_string();

                match self.color {
                    Color::Full => {
                        current_char = current_char
                            .truecolor(*r, *g, *b)
                            .on_truecolor(*r, *g, *b)
                            .to_string();
                    }
                    Color::Char => {
                        current_char = current_char.truecolor(*r, *g, *b).to_string();
                    }
                    Color::Background => {
                        current_char = current_char.on_truecolor(*r, *g, *b).to_string();
                    }
                    Color::Gray => {
                        let gray_code =
                            (*r as f32 * 0.3 + *g as f32 * 0.59 + *b as f32 * 0.11) as u8;
                        current_char = current_char
                            .truecolor(gray_code, gray_code, gray_code)
                            .to_string();
                    }
                    Color::NoColor => {}
                };

                if *ay > row_ptr {
                    row_ptr = *ay;
                    current_char = format!("\n{}", current_char);
                }

                current_char
            })
            .collect();

        write!(f, "{}", rended_textart)
    }
}

pub fn image2text_art_matrix(img: DynamicImage, options: Options) -> TextArtMatrix {
    let _terminal_size = terminal_size();

    let real2screen_cof: f32 = match _terminal_size {
        Some((Width(w), Height(h))) => {
            let real2screen_width_cof = (img.width() as f32) / w as f32;
            let real2screen_height_cof = (img.height() as f32) / h as f32;

            if real2screen_height_cof > real2screen_width_cof {
                real2screen_height_cof
            } else {
                real2screen_width_cof
            }
        }
        None => {
            eprintln!("Unable to get terminal size");
            1.0
        }
    };

    let new_width: u32 = match options.width {
        Some(width) => width,
        None => ((img.width() as f32 / real2screen_cof) * options.ratio) as u32,
    };

    let new_height: u32 = match options.width {
        Some(height) => height,
        None => ((img.height() as f32 / real2screen_cof) * options.ratio * CHAR_WIDTH) as u32,
    };

    let converted_image = img.resize_exact(new_width, new_height, imageops::FilterType::Gaussian);

    let pixels = converted_image
        .pixels()
        .into_iter()
        .map(|i| {
            let (ax, ay, Rgba([r, g, b, a])) = i;

            let saturation = (r as f64 + g as f64 + b as f64 + a as f64) / 1020 as f64;

            let chars_count = options.charset.chars().count() - 1;

            let mut char_index = (saturation * chars_count as f64) as usize;

            if options.invert {
                char_index = chars_count - char_index;
            }

            let current_char = options.charset.chars().nth(char_index).unwrap_or_else(|| {
                eprintln!(
                    "Something went error while we trying find char with index: '{}'",
                    char_index
                );
                exit(1)
            });

            Pixel((ax, ay), current_char, (r, g, b), saturation)
        })
        .collect();

    TextArtMatrix {
        pixels: pixels,
        color: options.color,
    }
}
