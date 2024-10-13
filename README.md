# Steganography Suite

### Note: The folder `stego-face` is the folder containing the GUI version of this project. It is also contains the most up to date version of the folders `original` and `testing_ground`

## WHAT IS THIS:

According to Kaspersky: Steganography is the practice of concealing information within another message or physical object to avoid detection. 

This app allows you (through a GUI) to encode and decode a secret into an image file.

But unlike most implementations, this one includes AES-GCM 256 bit encryption with a master password.

## WHY DID YOU MAKE THIS

I saw a youtube short of PirateSoftware talking about how he stores his passwords with steganography and I was like, you know that might actually be cool to implement with encryption then i could have my own local password manager based on just images.


## HOW TO USE:

Download the binary in the root project directory titled `stego` and run it

The saved image containing your message will be saved in the original directory of your file with a new file name "<old_file_name> (sneaky).<ext>"

See Video for Demo:

<video will be here sometime i needa make it look pretty before iwanna show ittt>

[DEMO](https://github.com/Chris-Coleongco/Steganography_Suite/blob/main/demo.mp4?raw=true)

## TECH USED:

crab lang (rust and the tauri framework for the GUI)

Javascript (for the front end)


## HOW IT WORKS:

First I made the encoding and decoding CLI. This was in the `original` folder. Here I took an ascii message and sequentially stored it's bits into the LSB (least significant bit) of every alpha channel byte (the one that controls opacity of a pixel). I had  the message length encoded into the last 32 pixels of the image.

Next I added encryption to the CLI via  the AES-GCM 256 bit algorithm provided by the rust crate aes-gcm. Lovely lovely crate hehe

Then it was finally time to slap on a GUI because it didn't feel as intuitive for me to use during testing lol

Why did you pick 32 bits to store the message length?

yea i know 4_294_967_295 bits to state the message length seems overkill, but the next lowest one was u16 which woulda meant 65_535 bits which wouldn't be enough to fully maximize the space on a 1920x1080 image (this would result in a max capacity of well over 1_000_000 bits).

