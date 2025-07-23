# 🕵️ Steganography Suite

This project is a fully local steganography and encryption tool that lets you encode and decode secret messages inside image files — complete with a modern GUI.

Unlike most stego tools, this one adds an extra layer of **AES-GCM 256-bit encryption**, secured by a master password. In other words: not only is your data hidden inside an image — it’s also encrypted.

---

## 🗂 Note on Structure

The folder `stego-face/` contains the **GUI version** of the app and is the **most up-to-date**. It also includes:

- `original/` – the original CLI version
- `testing_ground/` – for experiments and development

---

## ❓ What Is This?

> *“Steganography is the practice of concealing information within another message or physical object to avoid detection.”*  
> — Kaspersky

This app lets you **encode and decode hidden messages inside images**. It supports:

- **Message hiding** in the least significant bits of image pixels
- **AES-256-GCM encryption** using a password
- **Cross-platform GUI** (built with Tauri + JS frontend)

So yes, it’s kind of like a **password manager that hides your vault in a PNG**.

---

## 🧠 Why I Built This

I wanted a completely local, lightweight, encrypted password manager...  
But one that uses **images** as the storage format.  
So I built my own tool using Rust, Tauri, and some LSB magic.

---

## 🖱 How to Use

1. Download the binary named `stego` from the root directory.
2. Run the app.
3. Use the GUI to:
   - **Encode** a secret message into an image.
   - **Decode** a message from an image (with your password).

### 🔄 Output

When you encode a message, the new image will be saved in the **same directory as the original** — but renamed to:

<original_filename> (sneaky).<extension>


Example:  
`passwords.png` → `passwords (sneaky).png`

---

## 🧪 How It Works

### 🔐 Step 1 – Encryption

Before encoding, the message is **encrypted using AES-256-GCM**, provided by the [Rust `aes-gcm`](https://docs.rs/aes-gcm/latest/aes_gcm/) crate.

- Encryption/decryption is done locally via PBKDF2 password to symmetric key derivation.

### 🖼️ Step 2 - Encoding

- Read the given message with bit-masking.
- Stores each bit into the **LSB of the alpha channel** (opacity byte) of each pixel.
- The **last 32 pixels** of the image are reserved for the message length (in bits).
(looking back this was a weird implementation, size should be first ideally)


### 🎨 Step 3 – GUI (in `stego-face/`)

The GUI wraps the encoding/decoding flow into a friendly frontend using:

- **Rust + Tauri** for the backend
- **JavaScript** for the frontend (Tauri handles bridging)

It makes testing and using the tool way more intuitive.

---

## 🧮 Why 32 Bits for Message Length?

Yes, 32 bits lets us encode up to `4_294_967_295` bits for the message length — probably overkill. But `u16` (65,535 bits) would have limited the usable space in something like a 1920×1080 image.

With `u32`, we maximize compatibility with high-res images and leave room for long encrypted payloads.

---

## 🧰 Tech Stack

| Layer       | Tech                      |
|-------------|---------------------------|
| Backend     | Rust, `aes-gcm` crate     |
| GUI Runtime | Tauri                     |
| Frontend    | JavaScript/JSX (Tauri frontend) |
| Image I/O   | Rust image libraries      |
| Crypto      | AES-GCM 256-bit encryption |
| Format      | PNGs with alpha channel   |
