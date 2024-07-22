use bincode;
use inline_colorization::*;
use serde::Deserialize;
use sha1_smol::Sha1;
use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Write},
    process::exit,
};

const PATCH_DATA: &[u8] = include_bytes!("..\\res\\patches.bin");

fn print_logo() {
    println!(include_str!("..\\res\\banner.txt"), color_yellow=color_yellow, color_reset=color_reset);
}

#[derive(Deserialize)]
struct Patch {
    offset: usize,
    data: Vec<u8>,
}

#[derive(Deserialize)]
struct Version {
    name: String,
    patches: Vec<Patch>,
    final_hexdigest: String,
}

fn load_patches() -> HashMap<&'static str, Version> {
    bincode::deserialize(PATCH_DATA).expect("Couldn't load embedded patches, program is probably corrupted.")
}

fn write_file(file: &mut File, buffer: &mut Vec<u8>) {
    match file.write_all(buffer) {
        Ok(_) => (),
        Err(_) => fail("Unknown write error."),
    }
}

fn fail(message: &str) {
    println!("{color_red}=> {}{color_reset}", message);
    exit(-1);
}

fn main() {
    let versions = load_patches();

    if env::args().len() != 3 {
        println!("Usage: patcher.exe [ORIGINAL DLL] [NEW DLL]");
        exit(-1);
    }
    print_logo();

    let original_filename = env::args().nth(1).unwrap();
    let new_filename = env::args().nth(2).unwrap();
    let mut file_data: Vec<u8> = Vec::new();

    println!("=> Reading {}...", original_filename);

    match fs::read(&original_filename) {
        Ok(data) => file_data = data,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => fail("File doesn't exist"),
            ErrorKind::PermissionDenied => fail("Insufficient permissions, make sure you have access to the file."),
            _ => fail("Unknown error while opening file"),
        },
    }

    println!("=> Checking hash...");
    let mut hash = Sha1::from(&file_data).hexdigest();
    if !versions.contains_key(hash.as_str()) {
        fail("Unknown version. Either your version is not supported or the file has been modified. Open an issue on GitHub.")
    }

    let current_version = versions.get(hash.as_str()).unwrap();
    println!("=> Hash OK. Detected version: {}", current_version.name);

    for (idx, patch) in current_version.patches.iter().enumerate() {
        println!("{color_yellow}=> Patching {} of {}...{color_reset}", idx + 1, current_version.patches.len());
        file_data.splice(
            patch.offset..patch.offset + patch.data.len(),
            patch.data.clone(),
        );
    }

    println!("=> Checking hash...");
    hash = Sha1::from(&file_data).hexdigest();
    if current_version.final_hexdigest != hash.as_str() {
        fail("Unknown error. Open an issue on GitHub.");
    }

    println!("=> Hash OK.");
    println!("=> Writing patched file to {}...", new_filename);

    let output_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(new_filename);

    match output_file {
        Ok(mut file) => write_file(&mut file, &mut file_data),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => fail("File already exists."),
            ErrorKind::PermissionDenied => fail("Insufficient permissions, make sure you have access to the file."),
            _ => fail("Unknown I/O Error")
        },
    }
    println!("{color_green}Patcher done.{color_reset}");
}
