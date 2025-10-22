# rustyocr
A lightweight **Rust-based OCR tool** that lets you capture a region of your screen and extract its text directly to the clipboard (fast, simple, and native for Wayland).

---

## Description

`rustyocr` is a command-line utility written in Rust that captures a screenshot using **grim** and **slurp**, processes it through **Tesseract OCR**, and automatically copies the recognized text to your clipboard via **wl-clipboard**.

It’s designed for minimalism and efficiency, making it a handy companion for quickly grabbing text from images, documents, or the web.

---

## Dependencies

Make sure you have the following system dependencies installed:

- `tesseract`
- `tesseract-data-eng` (for English OCR support)
- `grim`
- `slurp`
- `wl-clipboard`

> 🔤 You can add additional languages by installing `tesseract-ocr-YOUR_LANG_CODE` or `tesseract-data-YOUR_LANG_CODE`
> Example: `tesseract-data-por` for Portuguese.

---

## Installation

### Arch Linux (via PKGBUILD)

If you’re on Arch Linux or a derivative (like Manjaro), you can install `rustyocr` using the provided `PKGBUILD`.

```
git clone https://github.com/rodrig20/rustyocr.git
cd rustyocr
makepkg -si
```

Once installed, you can run it from anywhere:

```
rustyocr
```

---

### Other Linux Distros (via Cargo)

If your distro isn’t Arch-based, you can install it using Rust’s package manager.

1. Make sure Rust and Cargo are installed:

2. Clone the repository and install it globally for your user:

```
git clone https://github.com/rodrig20/rustyocr.git
cd rustyocr
cargo install --path .
```

This installs `rustyocr` into your `~/.cargo/bin` directory.
Make sure this directory is in your `$PATH`:

```
export PATH="$HOME/.cargo/bin:$PATH"
```

You can now run:

```
rustyocr
```

---

## Language Usage

By default, rustyocr uses English (`eng`).
You can specify another OCR language as a command-line argument:

```
rustyocr por
```

If the language is not installed, rustyocr will display an error message and show available options from `/usr/share/tessdata`.

---

## License

This project is licensed under the **MIT License**.
See the `LICENSE` file for details.

---

Made with 🦀 Rust and ❤️ for the Linux community.
