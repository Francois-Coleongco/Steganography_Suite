fn main() {
    let byte: u8 = 0b1010_1010; // Example byte

    // Iterate over each bit position from 0 to 7 (since u8 has 8 bits)
    for i in 0..8 {
        // Check if the i-th bit is set (1) or not (0)
        let bit_value = (byte >> i) & 1;
        
        println!("Bit {} is: {}", i, bit_value);
    }
}

//msg_bytes

//for byte in image // one bit will be written to it
//msg_bytes[index] in message 8 bits to write
//for msg_bit in msg_byte
//write bit to lsb of img_byte
//
//
//msg_bytes[]
//
//
//index = 0
//
//for msg_byte in msg_bytes
//for bit in msg_byte
//for byte in image
//write bit
