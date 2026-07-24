use aes_gcm::{
    aead::{generic_array::GenericArray, AeadCore},
    Aes256Gcm,
};
use image::{save_buffer, ColorType::Rgba8, GenericImageView, ImageBuffer, Rgba};

const LENGTH_HEADER_PIXELS: u32 = 32;

fn set_lsb(pixel: &mut Rgba<u8>, bit: u8) {
    if bit == 1 {
        pixel.0[3] |= 0b0000_0001;
    } else {
        pixel.0[3] &= 0b1111_1110;
    }
}

fn encode_alpha(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    message: &[u8],
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let (width, height) = img.dimensions();
    let total_pixels = width * height;
    let required_pixels = (message.len() as u32) * 8 + LENGTH_HEADER_PIXELS;

    if required_pixels > total_pixels {
        return Err(format!(
            "Payload too large: need {} pixels, image has {}",
            required_pixels, total_pixels
        ));
    }

    let mut out = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    // Write length header into the first 32 pixels (MSB-first)
    let mut length_bits = Vec::with_capacity(32);
    for i in (0..32).rev() {
        length_bits.push(((message.len() >> i) & 1) as u8);
    }

    for (i, bit) in length_bits.iter().enumerate() {
        let x = i as u32;
        let mut pixel = *img.get_pixel(x, 0);
        set_lsb(&mut pixel, *bit);
        out.put_pixel(x, 0, pixel);
    }

    // Write body starting from pixel 32 (LSB-first per byte)
    let mut bit_values = Vec::with_capacity(message.len() * 8);
    for byte in message {
        for i in 0..8 {
            bit_values.push((byte >> i) & 1);
        }
    }

    for (i, bit) in bit_values.iter().enumerate() {
        let flat_index = (LENGTH_HEADER_PIXELS as usize) + i;
        let x = (flat_index as u32) % width;
        let y = (flat_index as u32) / width;
        let mut pixel = *img.get_pixel(x, y);
        set_lsb(&mut pixel, *bit);
        out.put_pixel(x, y, pixel);
    }

    Ok(out)
}

fn decode_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(Vec<u8>, u32), String> {
    let (width, _height) = img.dimensions();

    // Read length header from the first 32 pixels
    let mut storage: u32 = 0;
    for i in 0..LENGTH_HEADER_PIXELS {
        let pixel = img.get_pixel(i, 0);
        let bit = (pixel.0[3] & 1) as u32;
        storage = bit | (storage << 1);
    }

    let byte_length = storage as usize;

    // Read body pixels
    let mut bits = Vec::with_capacity(byte_length * 8);
    for i in 0..byte_length * 8 {
        let flat_index = LENGTH_HEADER_PIXELS as usize + i;
        let x = (flat_index as u32) % width;
        let y = (flat_index as u32) / width;
        let pixel = img.get_pixel(x, y);
        bits.push(pixel.0[3] & 1);
    }

    // Reconstruct bytes from bits (little-endian per byte)
    let mut final_bytes = Vec::with_capacity(byte_length);
    let mut current_byte: u8 = 0;
    let mut bit_count = 0;

    for bit in bits {
        current_byte = bit | (current_byte << 1);
        bit_count += 1;
        if bit_count == 8 {
            final_bytes.push(current_byte.reverse_bits());
            current_byte = 0;
            bit_count = 0;
        }
    }

    Ok((final_bytes, storage))
}

pub fn encoder(
    master_salt: [u8; 16],
    nonce: GenericArray<u8, <Aes256Gcm as AeadCore>::NonceSize>,
    ciphertext: Vec<u8>,
    mut file_path: String,
) -> Result<(), String> {
    println!("[stego::encoder] opening image: {}", file_path);
    let image = image::open(&file_path).map_err(|e| format!("Failed to open image: {}", e))?;
    let img_rgba = image.to_rgba8();
    let (width, height) = image.dimensions();
    println!("[stego::encoder] image dimensions: {}x{}", width, height);

    let mut payload = master_salt.to_vec();
    payload.extend(nonce);
    payload.extend(&ciphertext);
    println!("[stego::encoder] payload len: {} bytes (salt 16 + nonce 12 + ciphertext {})", payload.len(), ciphertext.len());

    let new_image_buffer = encode_alpha(img_rgba, &payload)?;

    if let Some(pos) = file_path.rfind(".") {
        file_path.insert_str(pos, ".stego");
    } else {
        return Err("File has no extension".to_string());
    }

    println!("[stego::encoder] saving to: {}", file_path);
    save_buffer(&file_path, &new_image_buffer, width, height, Rgba8)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    println!("[stego::encoder] done");
    Ok(())
}

pub fn decoder(file_path: &str) -> Result<Vec<u8>, String> {
    println!("[stego::decoder] opening image: {}", file_path);
    let image = image::open(file_path).map_err(|e| format!("Failed to open image: {}", e))?;
    let img_rgba = image.to_rgba8();

    let (data, length) = decode_alpha(img_rgba)?;
    println!("[stego::decoder] decoded {} bytes from {} pixel bits", data.len(), length * 8);
    Ok(data)
}
