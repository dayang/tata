use image::DynamicImage::ImageRgb8;
use image::{Rgb, RgbImage, ImageFormat};
use imageproc::drawing::{
    draw_text_mut, 
    draw_filled_rect_mut, 
    draw_line_segment_mut, 
    draw_cubic_bezier_curve};
use imageproc::noise::gaussian_noise_mut;
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};

use rand::{thread_rng, Rng};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct Captcha {
    width: u32,
    height: u32,
    chars: u32,
}

pub struct CaptchaResult {
    pub captcha_code: String,
    /// captcha image buffer with .png format
    pub img_buffer: Vec<u8>
}

impl CaptchaResult {
    /// extension must be .png now
    pub fn save_png<P: AsRef<Path>>(self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.img_buffer)
    }
}

impl Captcha {
    fn coerce_font_size(&self) -> f32
    {
        std::cmp::min(self.height, self.width / self.chars) as f32
    }

    pub fn draw(&self) -> CaptchaResult{
        let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

        let mut rng = thread_rng();
        let mut image = RgbImage::new(self.width, self.height);

        let light_red = rng.gen_range(160, 210);
        let light_green = rng.gen_range(160, 210);
        let light_blue = rng.gen_range(160, 210);

        draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(self.width, self.height), Rgb([light_red, light_green, light_blue]));

        let mut captcha_code = String::new();
        for _ in 0..self.chars{
            captcha_code.push(match rng.gen_range(0, 3) {
                0 => (rng.gen_range(0, 10) + b'0') as char,
                1 => (rng.gen_range(0, 26) + b'a') as char,
                2 => (rng.gen_range(0, 26) + b'A') as char,
                _ => unreachable!()
            });
        }
        
        let font_size = self.coerce_font_size();
        
        let scale = Scale {
            x: font_size,
            y: font_size,
        };

        for (i, ch) in captcha_code.chars().enumerate() {
            let deep_font_color = Rgb([rng.gen_range(0, 140), rng.gen_range(0, 140), rng.gen_range(0, 140)]);

            let bubble_px = font_size / 6.0;
            let mut x = i as f32 * font_size + rng.gen_range((bubble_px * -1.0) as f64, bubble_px as f64 ) as f32;
            if x < 0.0 {
                x = 2.0;
            }
            let mut max_y = self.height as f32 - font_size;
            if max_y <= 1.0 {
                max_y = 1.0;
            }
            let y = rng.gen_range(0.0, max_y as f64) as f32;

            draw_text_mut(
                &mut image,
                deep_font_color,
                x as u32,
                y as u32,
                scale,
                &font,
                &ch.to_string(),
            );
        }

        for _ in 0..rng.gen_range(1,3){
            let deep_line_color = Rgb([rng.gen_range(0, 140), rng.gen_range(0, 140), rng.gen_range(0, 140)]);

            let start_point = (rng.gen_range(0, self.width) as f32, rng.gen_range(0, self.height) as f32);
            let end_point = (rng.gen_range(0, self.width) as f32, rng.gen_range(0, self.height) as f32);
            draw_line_segment_mut(&mut image, start_point, end_point, deep_line_color);
        }

        for _ in 0..rng.gen_range(1,2) {
            let deep_line_color = Rgb([rng.gen_range(0, 140), rng.gen_range(0, 140), rng.gen_range(0, 140)]);

            let start_point = (rng.gen_range(0, self.width) as f32, rng.gen_range(0, self.height) as f32);
            let end_point = (rng.gen_range(0, self.width) as f32, rng.gen_range(0, self.height) as f32);
            let bezier_point1 = (rng.gen_range(0, self.width) as f32, rng.gen_range(0, self.height) as f32);
            let bezier_point2 = (rng.gen_range(0, self.width) as f32, rng.gen_range(0, self.height) as f32);

            image = draw_cubic_bezier_curve(&image, start_point, end_point, bezier_point1, bezier_point2, deep_line_color);
        }

        gaussian_noise_mut(
            &mut image,
            30.0,
            15.0,
            10
        );

        let mut img_buffer = vec![];
        let _ = ImageRgb8(image).write_to(&mut img_buffer, ImageFormat::Png);

        CaptchaResult {
            captcha_code: captcha_code,
            img_buffer: img_buffer
        }
    }
}

/// CaptchaBuilder
pub struct CaptchaBuilder {
    inner_captcha: Captcha
}

impl CaptchaBuilder {
    pub fn default() -> Self {
        CaptchaBuilder {
            inner_captcha: Captcha {
                width: 50,
                height: 50,
                chars: 4,
            }
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.inner_captcha.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.inner_captcha.height = height;
        self
    }

    pub fn chars(mut self, chars: u32) -> Self {
        self.inner_captcha.chars = chars;
        self
    }

    pub fn build(self) -> Captcha {
        self.inner_captcha
    }
}

#[cfg(test)]
mod tests {
    use Super::*;
    #[test]
    fn it_works() {
        let captcha = CaptchaBuilder::default()
        .width(100)
        .height(25)
        .chars(4)
        .build();

        match captcha.draw().save_png("captcha.png") {
            Err(s) => println!("{}", &s),
            _ => ()
        }
    }
}
