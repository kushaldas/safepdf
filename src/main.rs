use std::process::Command;
use std::path::Path;
use std::fs;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let _matches = App::new("safepdf")
        .version("0.1.0")
        .author("Kushal Das <mail@kushaldas.in>")
        .about("Convert PDF to safer PDF.")
        .arg(Arg::with_name("file")
                 .short("c")
                 .long("convert")
                 .takes_value(true)
                 .help("The file to convert"))
        .get_matches();
    
    let filepath = _matches.value_of("file").unwrap_or("noclue");
    if filepath == "noclue" {
        println!("Missing PDF file to convert.")
    }
    else if !filepath.ends_with(".pdf") {
        println!("Error: We need a PDF file to convert.")
    }
    let fpath = Path::new(filepath);
    if !fpath.exists() {
        println!("The PDF file {} is missing.", filepath)
    }

    // Let us create the new diretory for images
    let _res = fs::create_dir("./temp_images");
    let stem_name = fpath.file_stem().unwrap().to_string_lossy();
    let output_file = format!("./{}-final.pdf", stem_name);
    let image_name = format!("./temp_images/{}-%05d.png", stem_name);
    
    // Now let us blow up the pdf
    let output = Command::new("gm").arg("convert")
                        .arg("-density")
                        .arg("128")
                        .arg(filepath)
                        .arg("-quality")
                        .arg("100")
                        .arg("-sharpen")
                        .arg("0x1.0")
                        .arg("+adjoin")
                        .arg(image_name).output().unwrap();


    if !output.status.success() {
        panic!("convert executed with failing error code");
    }
    
    // Time to create the output pdf
    Command::new("gm").arg("convert").arg(format!("./temp_images/{}-*.png", stem_name))
                           .arg(output_file).output().unwrap();

    // Now let us remove the temporary directory
    fs::remove_dir_all("./temp_images").unwrap();
}