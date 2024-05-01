use image::{open, ImageBuffer, Rgb};
use crossterm::{
    execute, style::{self, Color, Print}, ExecutableCommand
};
use std::io::{stdout, Write};
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
use std::error::Error;


fn reset_terminal() {
    execute!(stdout(), style::ResetColor).unwrap();
}

fn print_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut stdout = stdout();
    let width = img.width();
    let hight = img.height();

    // print two lines at once, use background color for top line and foreground color for bottom line
    for y in (0..hight).step_by(2) {
        for x in 0..width {
            let top = img.get_pixel(x, y);
            let bottom = img.get_pixel(x, y + 1);

            let top_color = top.0;
            let bottom_color = bottom.0;

            let top_color = Color::Rgb {
                r: top_color[0],
                g: top_color[1],
                b: top_color[2],
            };

            let bottom_color = Color::Rgb {
                r: bottom_color[0],
                g: bottom_color[1],
                b: bottom_color[2],
            };

            stdout
                .execute(style::SetBackgroundColor(top_color))
                .unwrap()
                .execute(style::SetForegroundColor(bottom_color))
                .unwrap()
                .execute(Print("▄"))
                .unwrap();
        }
        stdout.execute(style::ResetColor)
            .unwrap()
            .execute(Print("\n"))
            .unwrap();
    }

    stdout.flush().unwrap();
    
}

fn load_image(path: &str) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
    let img = open(path)?;
    let img = img.to_rgb8();
    Ok(img)
}

fn scale_to_terminal_dimensions(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (w, h) = img.dimensions();
    let (terminal_width, _terminal_height) = crossterm::terminal::size().unwrap();
    let scale = terminal_width as f32 / w as f32;
    let height = (h as f32 * scale) as u32;
    let width = terminal_width as u32;
    let scaled = image::imageops::resize(img, width, height, image::imageops::FilterType::Nearest);
    scaled
}

fn main() {
    let img = load_image("test.png").unwrap();

    let img = scale_to_terminal_dimensions(&img);
    print_image(img);
    reset_terminal();
}