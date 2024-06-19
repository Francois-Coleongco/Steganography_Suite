use image::{save_buffer, DynamicImage, GenericImageView, ImageBuffer, Rgba, ColorType::Rgba8};


fn encode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>, message: &[u8]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    let (width, height) = img.dimensions();
    let bytes = width * height;

    if message.len() > bytes as usize{
        panic!("Input is too large for image size");
    }

    let mut out = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    
    for (x, y, pixel) in img.enumerate_pixels() {
        let mut tmp_pixel = pixel.clone();
        
        let input_index = x + (y * width);
        
        if input_index < message.len() as u32{
            tmp_pixel.0[3] = message[input_index as usize]; // 4th item (start at 0)
        }

        out.put_pixel(x, y, tmp_pixel.clone());
    }

    return out;
}

fn main() {

    let mut filepath = String::new();

    std::io::stdin().read_line(&mut filepath).expect("couldn't read input");

    filepath = filepath.trim().to_string();

    let image = image::open(filepath).expect("error opening");

    let img_as_rgba = image.to_rgba8();

    let (width, height) = image.dimensions();

    save_buffer("test.png", &img_as_rgba, width, height, Rgba8).expect("failed to copy");

    
}