use std::{collections::HashSet, io::Read};
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
        
        let input_index = x + (y * width); // counts left to right and from top to down to calculate the total pixels encompassed which corresponds to the number of alpha channels that can be lsb written to
        
        if input_index < message.len() as u32{
            tmp_pixel.0[3] = message[input_index as usize]; // 4th item (start at 0)
        }

        out.put_pixel(x, y, tmp_pixel.clone());
    }

    return out;
}

fn unique_elements<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

fn decode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();

    for (_, _, pixel) in img.enumerate_pixels() {
        out.push(pixel.0[3]);
    }

    out
}

fn main() {

    let null_byte = "\0".as_bytes();

    let mut filepath = String::new();

    println!("enter filepath of image: ");

    std::io::stdin().read_line(&mut filepath).expect("couldn't read input");

    filepath = filepath.trim().to_string();

    let image = image::open(filepath).expect("error opening");

    let img_as_rgba = image.to_rgba8();

    let (width, height) = image.dimensions();

    save_buffer("direct_copy.png", &img_as_rgba, width, height, Rgba8).expect("failed to direct-copy");

    let message = "i need to peee".as_bytes();


    let new_image_buffer = encode_alpha(img_as_rgba, message);

    save_buffer("message_copy.png", &new_image_buffer, width, height, Rgba8).expect("failed to message-copy");


    let data = decode_alpha(new_image_buffer);

    let clean_buffer: Vec<u8> = data.into_iter()
                                    .filter(|b| {
                                        *b != 0xff_u8
                                    })
                                    .collect();

    let mut a = clean_buffer.as_slice();

    let mut newbuff = String::new();

    println!("bytes found: {}", a.read_to_string(&mut newbuff).expect("oh no")); // it did get in. this is whats being pritned. it says the number of bytes aka the number of chars in the message written
    println!("{}", std::str::from_utf8(a).expect("oh no"));

    
}