use crossterm::{cursor, terminal, ExecutableCommand};
use image::{ImageBuffer, Pixel, Rgb};
use nokhwa::{Camera, CameraFormat, FrameFormat};
use std::io::{stdin, stdout};

static ASCII_MAP: &str = " .,-~+=@";
//static ASCII_MAP: &str = ".,:-+=#%@";
static SCALE: u32 = 5;

fn ascii_value(rgb: &Rgb<u8>) -> char {
    let luma = rgb.to_luma().0[0];

    let n: usize = ((luma as f32 / 255f32) * ASCII_MAP.len() as f32).floor() as usize;
    // println!(
    //     "LUMA: {} , NORMALISED LUMA : {} , LEN: {} N: {}",
    //     luma,
    //     luma / 255,
    //     ascii_map.len(),
    //     n
    // );
    // println!("luma: {} n: {}", luma, n);
    if let Some(a) = ASCII_MAP.chars().nth(n) {
        a
    } else {
        '@'
    }
}

fn get_ascii(frame: &ImageBuffer<Rgb<u8>, Vec<u8>>, scale: u32) -> String {
    let mut output = String::new();

    for (x, y, pixel) in frame.enumerate_pixels() {
        if y % (2 * scale) == 0 && x % scale == 0 {
            output += &ascii_value(pixel).to_string();
            if x == 0 {
                output += "\n"
            }
        }
    }

    output
}

fn main() {
    println!("Enter Scale Number");
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let scale = match trimmed.parse::<u32>() {
        Ok(i) => match i {
            1..=10 => i,
            _ => SCALE,
        },
        Err(..) => SCALE,
    };

    let mut camera = Camera::new(
        0,                                                              // index
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();

    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    // open stream
    camera.open_stream().unwrap();
    loop {
        let frame: ImageBuffer<Rgb<u8>, Vec<u8>> = camera.frame().unwrap();

        let buffer: String = get_ascii(&frame, scale);

        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        println!("{}", buffer);
    }
}
