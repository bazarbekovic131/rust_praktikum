extern crate pdf_extract;
extern crate tts;

use std::fs::File;
use std::io::Write;

use pdf_extract::PdfExtract;
use tts::TTS;

fn main() {
    // Path to the input PDF file
    let pdf_path = "input.pdf";

    // Path to the output MP3 file
    let mp3_path = "output.mp3";

    // Load the PDF file
    let pdf = PdfExtract::from_file(&pdf_path).unwrap();

    // Extract text from the PDF
    let text = pdf.extract_text().unwrap();

    // Initialize the TTS engine
    let tts = TTS::new("voice_name").unwrap();

    // Generate the audio file
    let audio = tts.speak(&text).unwrap();

    // Save the audio as an MP3 file
    let mut output_file = File::create(&mp3_path).unwrap();
    output_file.write_all(&audio).unwrap();

    println!("Conversion complete. MP3 file saved at '{}'.", mp3_path);
}
