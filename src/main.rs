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
    let output_file = format!("./{}-safe.pdf", stem_name);
    let image_name = format!("./temp_images/{}_transparent.png", stem_name);
    
    // Now let us blow up the pdf
    let output = Command::new("convert").arg("-density")
                        .arg("128")
                        .arg(filepath)
                        .arg("-quality")
                        .arg("100")
                        .arg("-sharpen")
                        .arg("0x1.0")
                        .arg(image_name).output().unwrap();


    if !output.status.success() {
        panic!("convert executed with failing error code");
    }
    
    // Let us get all the filenames into a vector
    let mut vec: Vec<String> = Vec::new();
    let mut newnames: Vec<String> = Vec::new();
    for entry in fs::read_dir("./temp_images/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let fullpath = path.as_os_str();
        vec.push(String::from(fullpath.to_string_lossy()));
        
    }

    // Now let us create flatten images
    for fname in vec.clone() {
        let path = Path::new(&fname);
        let fullpath = path.as_os_str();
        let new_filename = if path.ends_with("_transparent.png") {
            let new_name = path.file_name().unwrap().to_string_lossy();
            new_name.replace("_transparent", "-0")
            
        } else {
            let new_name = path.file_name().unwrap().to_string_lossy();
            new_name.replace("_transparent", "")
        };

        newnames.push(new_filename.clone());

        // Now let us flaten those images
        Command::new("convert").arg(fullpath)
                        .arg("-flatten")
                        .arg(format!("./temp_images/{}", new_filename)).output().unwrap();
        //fs::remove_file(fname).unwrap();
    }

    // Now let us remove the old files
    for fname in vec {
        fs::remove_file(fname).unwrap();
    }

    for fname in newnames {
        let parts: Vec<&str> = fname.rsplitn(2, "-").collect();
        let len = parts[0].len();
        let required_len = 10 - len;
        
        // calculate the required 0s
        let ch = "0";
        let mut newstr = String::new();
        for i in 1..required_len {
            newstr.push_str(ch);
        }
        let mv_name = format!("./temp_images/{}-{}{}", parts[1], newstr, parts[0] );

        //rename now
        fs::rename(format!("./temp_images/{}", fname), mv_name).unwrap();
    }

    // Time to create the output pdf
    Command::new("convert").arg(format!("./temp_images/{}-*.png", stem_name))
                           .arg(output_file).output().unwrap();

    // Now let us remove the temporary directory
    fs::remove_dir_all("./temp_images").unwrap();
}