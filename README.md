# Steganography Suite

This project is a fully local steganography and encryption tool that lets you encode and decode secret messages inside image files in a desktop GUI.

Unlike most stego tools, this one adds an extra layer of **AES-GCM 256-bit encryption**, secured by a master password. In other words: not only is your data hidden inside an image, it’s also encrypted.

---

## What Is This?

> *“Steganography is the practice of concealing information within another message or physical object to avoid detection.”*  
> — Kaspersky

This app lets you **encode and decode hidden messages inside images**. It supports:

- **Message hiding** in the least significant bits of image pixels
- **AES-256-GCM encryption** using a password
- **GUI** (built with Tauri + JS frontend)

With this, you can have a **password manager that hides your passwords in a PNG** :)

---

## Tech Stack

| Layer       | Tech                      |
|-------------|---------------------------|
| Backend     | Rust, `aes-gcm` crate     |
| GUI Runtime | Tauri                     |
| Frontend    | JavaScript/JSX (Tauri frontend) |
| Image I/O   | Rust image libraries      |
| Crypto      | AES-GCM 256-bit encryption |
| Format      | PNGs with alpha channel   |

---

## Usage

1. Download the binary named `stego` from the releases.
2. Run the app.
3. Use the GUI to:
   - **Encode** a secret message into an image.
   - **Decode** a message from an image (with your password).

### Output

When you encode a message, the new image will be saved in the **same directory as the original** but renamed to:

<original_filename> (sneaky).<extension>


Example:  
`passwords.png` → `passwords (sneaky).png`

---

## How It Works


### Step 1 – Encryption

Before encoding, the message is **encrypted using AES-256-GCM**, provided by the [Rust `aes-gcm`](https://docs.rs/aes-gcm/latest/aes_gcm/) crate.

- Encryption/decryption is done locally via PBKDF2 password to symmetric key derivation.


### Step 2 - Encoding

- Read the given message with bit-masking.
- Stores each bit into the **LSB of the alpha channel** (opacity byte) of each pixel.
- The **last 32 pixels** of the image are reserved for the message length (in bits).
(looking back this was a weird implementation, size should be first ideally)


### Step 3 – GUI (in `stego-face/`)

The GUI wraps the encoding/decoding flow into a friendly frontend using:

- **Rust + Tauri** for the backend
- **JavaScript** for the frontend (Tauri handles bridging)
