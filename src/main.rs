use std::process;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;

extern crate clap;
extern crate image;

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
        .arg(Arg::with_name("explode")
                 .short("e")
                 .long("explode")
                 .help("Only convert to png files"))
        .arg(Arg::with_name("join")
                 .short("j")
                 .long("join")
                 .help("Join the png files to the final pdf file."))
        .get_matches();
    
    // Only when we already exploded the pdf before.
    if _matches.is_present("join") {
        join_pdf();
        process::exit(0);
    }

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
    let stem_name_str = format!("{}", stem_name);
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
    
    if _matches.is_present("explode") {
        let mut file = File::create("./temp_images/filename.txt").unwrap();
        file.write(stem_name_str.as_bytes()).unwrap();
        process::exit(0);
    }

    // We are also joining the pdf in one go.
    // We should do this in steps using separate containers.
    join_pdf();

}

// To verify all the PNG files in the directory
// This needs more work.
fn verify_pngs(){

    // Let us go through all the PNG files.
    for entry in fs::read_dir("./temp_images/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let fullpath = path.as_os_str();
        let fullname = String::from(fullpath.to_string_lossy());
        if fullname.ends_with(".png") {
            //vec.push(fullname);
            let img = image::open(fullpath).unwrap();
            //let out = File::create(fullpath).unwrap();
            let _ = img.save(fullpath);
        }
    }
}

fn join_pdf(){
    let fpath = Path::new("./temp_images/filename.txt");
    if !fpath.exists() {
        println!("The PDF file {} is missing.", fpath.to_str().unwrap());
        process::exit(10);
    }

    // Let us verify the PNG files
    // TODO: This need more work.
    verify_pngs();

    // Let us read the filename
    let mut filename = String::new();
    let mut file = File::open("./temp_images/filename.txt").expect("Unable to open the file");
    file.read_to_string(&mut filename).expect("Unable to read the file");

    let output_file = format!("./{}-final.pdf", filename);

    // Time to create the output pdf
    Command::new("gm").arg("convert").arg(format!("./temp_images/{}-*.png", filename))
                           .arg(output_file).output().unwrap();

    // Now let us remove the temporary directory
    fs::remove_dir_all("./temp_images").unwrap();
}