use std::{collections::HashSet, io::Read, u8};
use image::{save_buffer, DynamicImage, GenericImageView, ImageBuffer, Rgba, ColorType::Rgba8};


fn encode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>, message: &[u8]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    let (width, height) = img.dimensions();
    let bytes = width * height;

    if message.len() > bytes as usize{
        panic!("Input is too large for image size");
    }

    let mut out = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
   
    let mut bit_values: Vec<u8> = Vec::new();

    for msg_byte in message {
        for i in 0..8 {
            let bit_value = (msg_byte >> i) & 1;
            bit_values.push(bit_value);
            println!("{}", bit_value); // this prints the bit values in little endian (starting
            // from least significant bit)
        }
    }

    println!("every bit: {}", bit_values.len()); // came out to 360 which is 45 chars * 8 bits per
    // char

//    we have all the bits now in an array. need to write one to each pixel by doing the // To change the LSB to 1:
  //  byte |= 0b0000_0001;

    // To change the LSB to 0:
    //byte &= 0b1111_1110;


    for (x, y, pixel) in img.enumerate_pixels() {
        let mut tmp_pixel = *pixel;
        
        let input_index = x + (y * width); // counts left to right and from top to down to calculate the total pixels encompassed which corresponds to the number of alpha channels that can be lsb written to
        
        if input_index < message.len() as u32{
            tmp_pixel.0[3] = message[input_index as usize]; // 4th item (start at 0) this is
            // writing a byte to a byte space aka the alpha channel for htis pixel is being
            // replaced with the message. i want to replace a bit only
            //
            
            //for msg_byte in msg_bytes
            //for bit in msg_byte
            //for byte in image
            //write bit
        }

        out.put_pixel(x, y, tmp_pixel);
    }

    out
}


fn decode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> { // this needs to be edited to
    // read the lsb of each alpha channel byte of a pixel then stick em all together to form a
    // series of bits
    let mut out: Vec<u8> = Vec::new();

    for (_, _, pixel) in img.enumerate_pixels() {
        out.push(pixel.0[3]);
    }

    out
}

fn decoder(message_copy: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let data = decode_alpha(message_copy);

    let clean_buffer: Vec<u8> = data.into_iter()
                                    .filter(|b| {
                                        *b != 0xff_u8
                                    })
                                    .collect();

    let mut a = clean_buffer.as_slice();

    let mut newbuff = String::new();

    println!("bytes found: {}", a.read_to_string(&mut newbuff).expect("oh no")); // it did get in. this is whats being pritned. it says the number of bytes aka the number of chars in the message written
    
    println!("decoded: {}", newbuff);

}

fn encoder(img: ImageBuffer<Rgba<u8>, Vec<u8>>, width: u32, height: u32) {

    save_buffer("direct_copy.png", &img, width, height, Rgba8).expect("failed to direct-copy");

    let message = "this is an encoded message whuwheuauhaaahhaha".as_bytes();

    if message.len() > (width * height).try_into().expect("couldn't convert to usize") {
        println!("too small image to embed with alpha channel encoding");
    }

    let new_image_buffer = encode_alpha(img, message);

    save_buffer("message_copy.png", &new_image_buffer, width, height, Rgba8).expect("failed to message-copy");
}

fn main() {

    let mut option = String::new();

    let mut filepath = String::new(); 

    println!("enter option: (e for encode, d for decode): ");

    std::io::stdin().read_line(&mut option).expect("unable to read option");

    option = option.trim().to_string();

    println!("enter filepath to image (encode): ");

    std::io::stdin().read_line(&mut filepath).expect("unable to read filepath");

    filepath = filepath.trim().to_string();

    let image = image::open(filepath).expect("error opening");

    let img_as_rgba: ImageBuffer<Rgba<u8>, Vec<u8>> = image.to_rgba8();

    let (width, height) = image.dimensions();

    if option == "e" {
        encoder(img_as_rgba, width, height)
    }
    else if option == "d" {
        decoder(img_as_rgba)
    }
        
}
