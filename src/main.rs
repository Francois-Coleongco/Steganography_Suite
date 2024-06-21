use std::io::Read;
use image::{save_buffer, GenericImageView, ImageBuffer, Rgba, ColorType::Rgba8};

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
//
//
//  
    // let encode_storage_alpha(message.len().try_into().expect("couldn't convert msg len to u32"), img.clone(), width, height);
    // returns 

    for (x, y, pixel) in img.enumerate_pixels() {

        let mut tmp_pixel = *pixel;
        
        let input_index = x + (y * width); // counts left to right and from top to down to calculate the total pixels encompassed which corresponds to the number of alpha channels that can be lsb written to
       
        let input_index_as_usize: usize = input_index.try_into().expect("unable to calculate input_index_as_usize");

        if input_index < bit_values.len() as u32{

            if bit_values[input_index_as_usize] == 1 {
                tmp_pixel.0[3] |= 0b0000_0001;
            }
            else if bit_values[input_index_as_usize] == 0 {
                tmp_pixel.0[3] &= 0b1111_1110;
            }
            else {
                println!("wtf just happened");
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


    let mut x2 = width - 33; // this changes
    let y2 = height - 1; // this doesn't. write contiguously on the same y value

    let mut size_vec: Vec<u8> = Vec::new();


    for i in (0..32).rev() {
        let bit_value = (message.len() >> i) & 1;
        size_vec.push(bit_value.try_into().expect("unable to convert storage bit as u8 from u32"));
        println!("storage: {}", bit_value); // this prints the bit values in little endian (starting
        // from least significant bit)
    } // prints every bit to add 


    let mut current_pixel = *(img.get_pixel(x2,y2));
    
    for bit in size_vec {
        // get pixel at start x and y | write bit to alpha channel | increment pixel x +=1

        if bit == 1 {
            current_pixel.0[3] |= 0b0000_0001;
        }
        else if bit == 0 {
            current_pixel.0[3] &= 0b1111_1110;
        }
        else {
            println!("wtf just happenedd 2");
        }

        x2 += 1;

        out.put_pixel(x2, y2, current_pixel);

    };

    out
}


fn decode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> (Vec<u8>, u32) { // this needs to be edited to
    // read the lsb of each alpha channel byte of a pixel then stick em all together to form a
    // series of bits
    let mut out: Vec<u8> = Vec::new();


    // read message length first
    //
    //


    
    for (x, y, pixel) in img.enumerate_pixels() { // need this to read only pixels up to a certain
        // limit
            let bit_value = (pixel.0[3]) & 1; // gets lsb
            out.push(bit_value);
        }

    println!("out len: {}", out.len());
    // out is an array of 0's and 1's. i need to convert this array back to bytes
    //
    //

    let mut current_4bytes: u32 = 0;

    let mut bit_count = 0;

    for bit in &out[(out.len() - 32)..] {
        let bit: u32 = *bit as u32;
        current_4bytes = bit | (current_4bytes << 1);
        println!("bit found wewewewe: {}", bit);
        println!("current_byte: {}", current_4bytes);
        bit_count += 1;
        if bit_count == 32 {
            bit_count = 0;
            println!("became 32");
        }
    }
   //  EVERY BYTE MUST BE FLIPPED SO IT GIVES A RIGHT NUMBER FOR ASCII
    println!("final current_4bytes: {}", current_4bytes);


    let mut final_bytes: Vec<u8> = Vec::new();

    let mut current_byte: u8 = 0;

    let mut bit_count = 0;

    for bit in &out {
        current_byte = bit | (current_byte << 1);
        bit_count += 1;
        if bit_count == 8 {
            final_bytes.push(current_byte.reverse_bits());
            current_byte = 0;
            bit_count = 0;
        }
    }

    (final_bytes, current_4bytes)

}



fn decoder(message_copy: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let (data, length) = decode_alpha(message_copy);

    //
    //

    let mut index = 0;

    let mut final_message_vec = &data[..length as usize];

    for i in final_message_vec {
        println!("i: {}", i)
    }

    let mut newbuff = String::new();
    
    final_message_vec.read_to_string(&mut newbuff).expect("couldn't read final_message_vec into newbuff");

    println!("bytes found: {}", length); // it did get in. this is whats being pritned. it says the number of bytes aka the number of chars in the message written
    
    println!("decoded: {}", newbuff);

}



fn encoder(img: ImageBuffer<Rgba<u8>, Vec<u8>>, width: u32, height: u32) {

    save_buffer("direct_copy.png", &img, width, height, Rgba8).expect("failed to direct-copy");

    let message = "aaaaaaaaaaaaaaaaaaaaaaaaaa".as_bytes();

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
