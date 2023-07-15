use image::RgbImage;
use image::{ ImageBuffer, Rgb };
use nalgebra::Complex;
use nalgebra::Normed;
fn main() {
    let width = 800;
    let heigth = 600;
    let scale_x = 3.0 / (width as f64);
    let scale_y = 3.0 / (heigth as f64);
    let mut img = RgbImage::new(width, heigth);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx = (x as f64) * scale_x - 1.5;
        let cy = (y as f64) * scale_y - 1.5;
        let c = Complex { re: -0.8, im: 0.156 };
        let value = julia(c, cx, cy);
        *pixel = Rgb([value, value, value]);
    }
    img.save("julia.png");
    println!("Hello, world!");
}

fn julia(c: Complex<f64>, x: f64, y: f64) -> u8 {
    let mut z = Complex { re: x, im: y };
    for i in 0..255 {
        if z.norm() > 2.0 {
            return i as u8;
        }
        z = z * z + c;
    }
    255
}
