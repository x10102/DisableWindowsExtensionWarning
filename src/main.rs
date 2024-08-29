use bincode;
use inline_colorization::*;
use pelite::pe64::{exports::Export, Pe, PeFile, imports::Import};
use sha1_smol::Sha1;
use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    process::exit,
};

const PATCH_DATA: &[u8] = include_bytes!("..\\res\\patches.bin");

#[allow(dead_code)]
const ORDINAL_NUMBER: u16 = 781;
#[allow(dead_code)]
const ORDINAL_OFFSET_1: u16 = 0;


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

#[allow(dead_code)]
fn find_offset(dll_data: &Vec<u8>) {

    let Ok(pe) = PeFile::from_bytes(&dll_data) else {
        fail("Error parsing PE header, DLL is probably corrupted.");
    };

    let exports_by = pe.exports().unwrap().by().unwrap();
    
    if let Ok(imports) = pe.imports() {

        for import in imports {
            let dll_name = import.dll_name().unwrap().to_string();
            if dll_name.contains("shlwapi") {
                let iat = import.iat().unwrap();
                let int = import.int().unwrap();

                for (va, import) in Iterator::zip(iat, int) {
                    if let Import::ByName { name, .. } = import.unwrap() {
                        if name.to_str().unwrap() == "ShellMessageBoxW" {
                            println!("{} found at 0x{:x}", name.to_str().unwrap(), va)
                        }
                    }
                }
            }
        }
    } else {
        println!("No imports found in the target file.");
    }

    if let Ok(Export::Symbol(rva)) = exports_by.ordinal(ORDINAL_NUMBER) {
        println!("RVA is 0x{:x}", rva.to_owned() as usize);


        let offset = rva.to_owned() as usize;
        println!(
            "Offset is: 0x{:x} relative to the start of the DLL file",
            offset
        );
    } else {
        println!("Function not found in the export table");
    }
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
