use image::{io::Reader, ImageResult, imageops::FilterType::CatmullRom};

fn main() -> ImageResult<()> {
    const BRIGHTNESS_CHARS: [char; 64] = [
        ' ', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '/' ,'t' ,'f' ,'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'
    ];

    let file_path = std::env::args().nth(1).expect("No file given");

    let mut image = Reader::open(file_path)?
        .with_guessed_format()?
        .decode()?;

    if image.width() > 1000 {
        image = image.resize(1400, image.height(), CatmullRom);
    }

    let (width, height) = (image.width(), image.height());
    let image = image.resize_exact(width, height/3, CatmullRom);
    let (width, height) = (image.width(), image.height());
    let image_bytes = image.as_bytes();
    let pixel_size = image_bytes.len() / (width * height) as usize;
    let pixels: Vec<u8> = image_bytes
        .chunks_exact(pixel_size)
        .map(|pixel| {
            (pixel.iter()
            .fold(0u32, |acc, n| {
                acc + u32::from(*n)
            }) / pixel.len() as u32) as u8
        })
        .collect();

    pixels.chunks_exact(width as usize).for_each(|chunk| {
        for pixel in chunk  {
            let brightness = pixel / 4;
            print!("{}", BRIGHTNESS_CHARS[brightness as usize]);
        };
        println!(" ");
    });

    Ok(())
}
