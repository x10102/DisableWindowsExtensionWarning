use bincode;
use inline_colorization::*;
use serde::Deserialize;
use sha1_smol::Sha1;
use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    process::exit,
};

const PATCH_DATA: &[u8] = include_bytes!("..\\res\\patches.bin");

fn print_logo() {
    println!(
        include_str!("..\\res\\banner.txt"),
        color_yellow = color_yellow,
        color_reset = color_reset
    );
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
    bincode::deserialize(PATCH_DATA)
        .expect("Couldn't load embedded patches, program is probably corrupted.")
}

fn write_file(filename: &str, data: &mut Vec<u8>) {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(filename);

    match file {
        Ok(mut file) => {
            if file.write_all(data).is_err() {
                fail("Unknown write error.")
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => fail("File already exists."),
            ErrorKind::PermissionDenied => {
                fail("Insufficient permissions, make sure you have access to the file.")
            }
            _ => fail("Unknown I/O Error"),
        },
    }
}

fn read_file(filename: &str, buffer: &mut Vec<u8>) {
    match fs::read(filename) {
        Ok(mut data) => buffer.append(&mut data),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => fail("File doesn't exist"),
            ErrorKind::PermissionDenied => {
                fail("Insufficient permissions, make sure you have access to the file.")
            }
            _ => fail("Unknown error while opening file"),
        },
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

    let args: Vec<String> = env::args().collect();
    let [_, original_filename, new_filename] = args.try_into().unwrap();
    let mut file_data: Vec<u8> = Vec::new();

    println!("=> Reading {}...", original_filename);

    read_file(&original_filename, &mut file_data);

    println!("=> Checking hash...");
    let hash = Sha1::from(&file_data).hexdigest();
    let mut current_version = None;

    match versions.get(hash.as_str()) {
        Some(version) => current_version = Some(version),
        None => fail("Unknown version. Either your version is not supported or the file has been modified. Open an issue on GitHub.")
    }

    let current_version = current_version.unwrap();

    println!("=> Hash OK. Detected version: {}", current_version.name);

    for (idx, patch) in current_version.patches.iter().enumerate() {
        println!(
            "{color_yellow}=> Patching {} of {}...{color_reset}",
            idx + 1,
            current_version.patches.len()
        );
        file_data.splice(
            patch.offset..patch.offset + patch.data.len(),
            patch.data.clone(),
        );
    }

    println!("=> Checking hash...");
    let hash = Sha1::from(&file_data).hexdigest();
    if current_version.final_hexdigest != hash.as_str() {
        fail("Unknown error. Open an issue on GitHub.");
    }

    println!("=> Hash OK.");
    println!("=> Writing patched file to {}...", new_filename);

    write_file(&new_filename, &mut file_data);

    println!("{color_green}Patcher done.{color_reset}");
}
