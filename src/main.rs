use std::{collections::HashSet, io::Read, u8, usize};
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

YOUCAN TELL IT HOW MANNY BITS TO READ IF YOU STORE THE MESSAGE LENGTH IN ONE OF THE PIXELS (uSE THE VERY LAST 32 PIXELS TO STORE A 4 BYTE INT TO STATE HOW MUCH MESSAGE CAN BE IN IT)
    

    for (x, y, pixel) in img.enumerate_pixels() {

        let mut tmp_pixel = *pixel;
        
        let input_index = x + (y * width); // counts left to right and from top to down to calculate the total pixels encompassed which corresponds to the number of alpha channels that can be lsb written to
       
        let input_index_as_usize: usize = input_index.try_into().expect("unable to calculate input_index_as_usize");

        if input_index < bit_values.len() as u32{

            if bit_values[input_index_as_usize] == 1 {
                tmp_pixel.0[3] |= 0b0000_0001;
            }
            if bit_values[input_index_as_usize] == 0 {
                tmp_pixel.0[3] &= 0b1111_1110;
            }

            println!("ran input_index =>  {}", input_index);

        }
            // 4th item (start at 0) this is
            // writing a byte to a byte space aka the alpha channel for htis pixel is being
            // replaced with the message. i want to replace a bit only
            //
            
            //for msg_byte in msg_bytes
            //for bit in msg_byte
            //for byte in image
            //write bit

        out.put_pixel(x, y, tmp_pixel);
    }


    out
}


fn decode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> { // this needs to be edited to
    // read the lsb of each alpha channel byte of a pixel then stick em all together to form a
    // series of bits
    let mut out: Vec<u8> = Vec::new();

    for (_, _, pixel) in img.enumerate_pixels() {
            let bit_value = (pixel.0[3]) & 1; // gets lsb
            out.push(bit_value);
        }

    println!("out len: {}", out.len());
    // out is an array of 0's and 1's. i need to convert this array back to bytes
    //
    //
    let mut final_bytes: Vec<u8> = Vec::new();

    let mut current_byte: u8 = 0;

    let mut bit_count = 0;

    for bit in out {
        current_byte = bit | (current_byte << 1);
        bit_count += 1;
        if bit_count == 8 {
            final_bytes.push(current_byte.reverse_bits());
            current_byte = 0;
            bit_count = 0;
        }
    }
   //  EVERY BYTE MUST BE FLIPPED SO IT GIVES A RIGHT NUMBER FOR ASCII


    final_bytes

}


fn decoder(message_copy: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let data = decode_alpha(message_copy);

    // 

    let mut a = &data[0..44];

    for i in a {
        println!(" --> {}", i)
    }

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
