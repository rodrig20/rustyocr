mod screenshot;
use tempfile::NamedTempFile;
use leptess::LepTess;
use wl_clipboard_rs::copy::{MimeType, Options, Source};
use std::{env, fs, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Set language
    let args: Vec<String> = env::args().collect();
    let lang = if args.len() > 1 { &args[1] } else { "eng" };

    // Check if the language exists in tessdata
    let tessdata_path = "/usr/share/tessdata";
    let lang_file = format!("{}/{}.traineddata", tessdata_path, lang);

    if !fs::metadata(&lang_file).is_ok() {

        eprintln!("Error: the language '{}' is not installed.", lang);
        eprintln!("Please install the corresponding Tesseract package.");

        // List available languages
        eprintln!("Available languages:");
        match fs::read_dir(tessdata_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let filename = entry.file_name();
                        let name = filename.to_string_lossy();
                        if name.ends_with(".traineddata") {
                            println!("  {}", name.trim_end_matches(".traineddata"));
                        }
                    }
                }
            }
            Err(_) => eprintln!("  Could not list available languages."),
        }

        process::exit(1);
    }

    // Save screenshot temporarily
    let temp_file = NamedTempFile::new()?;
    screenshot::save(temp_file.path().to_str().unwrap());

    // Config leptess (Tesseract)
    let mut lt = LepTess::new(Some(tessdata_path), lang)?;
    lt.set_image(temp_file.path().to_str().unwrap())?;

    // Copy to clipboard
    let text = lt.get_utf8_text()?;
    let opts = Options::new();
    opts.copy(Source::Bytes(text.to_string().into_bytes().into()), MimeType::Text)?;

    // Show
    println!("{text}");

    Ok(())
}
