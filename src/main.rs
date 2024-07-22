use std::{collections::HashMap, env, fs::{File, OpenOptions}, io::{ErrorKind, Read, Write}, process::exit};
use str_macro::str;
use inline_colorization::*;
use sha1_smol::Sha1;

fn print_logo() {
    println!("
╔══════════════════════════════════════════════════════════════════╗
║  ██████ ██████   █████   ██████ ██   ██ ████████ ███████ ██   ██ ║
║ ██      ██   ██ ██   ██ ██      ██  ██     ██    ██      ██  ██  ║
║ ██      ██████  ███████ ██      █████      ██    █████   █████   ║
║ ██      ██   ██ ██   ██ ██      ██  ██     ██    ██      ██  ██  ║
║  ██████ ██   ██ ██   ██  ██████ ██   ██    ██    ███████ ██   ██ ║
╠══════════════════════════════════════════════════════════════════╣
║ => https://github.com/x10102/DisableWindowsExtensionWarning      ║
║ => {color_yellow}USE AT YOUR OWN RISK{color_reset}                                          ║
╚══════════════════════════════════════════════════════════════════╝");
}

struct Patch {
    offset: usize,
    data: Vec<u8>,
}

struct Version {
    name: String,
    patches: Vec<Patch>,
    final_hexdigest: String,
}

fn get_versions() -> HashMap<&'static str, Version> {
    HashMap::from([
        ("6301dfa9754e7edc520c5d9eb3448fa38b3a4c05", Version {
            name: str!("Windows 10 Home 22H2 build 19045.4529"),
            final_hexdigest: str!("2c0b7460b796422e612443a080c1c823fb55106f"),
            patches: vec![
                Patch {
                    offset: 0x004DC663,
                    data: vec![0x90; 7]
                },
                Patch {
                    offset: 0x004DC68D,
                    data: vec![0x90; 10]
                },
            ]
        }),
        ("bc4f0e77101e62f17a83112378eed57ab32590db", Version {
            name: str!("Windows 10 Home 22H2 build 19045.4651"),
            final_hexdigest: str!("985aaf2d470339622dfd0c0535b3a6db67223e36"),
            patches: vec![
                Patch {
                    offset: 0x004DCEB3,
                    data: vec![0x90; 7]
                },
                Patch {
                    offset: 0x004DCEDD,
                    data: vec![0x90; 10]
                },
            ]
        }),
    ])
}

fn read_file(file: &mut File, buffer: &mut Vec<u8>) {
    match file.read_to_end(buffer) {
        Ok(_) => (),
        Err(_) => fail("Unknown read error.")
    }
}

fn write_file(file: &mut File, buffer: &mut Vec<u8>) {
    match file.write_all(buffer) {
        Ok(_) => (),
        Err(_) => fail("Unknown write error.")
    }
}

fn fail(message: &str) {
    println!("{color_red}=> {}{color_reset}", message);
    exit(-1);
}

fn main() {
    let versions = get_versions();

    if env::args().len() != 3 {
        println!("Usage: patcher.exe [ORIGINAL DLL] [NEW DLL]");
        exit(-1);
    }
    print_logo();

    let original_filename = env::args().nth(1).unwrap();
    let new_filename = env::args().nth(2).unwrap();
    let mut file_data: Vec<u8> = Vec::new();

    println!("=> Reading {}...", original_filename);
    let file = OpenOptions::new().read(true).open(&original_filename);
    match file {
        Ok(mut file) => read_file(&mut file, &mut file_data),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => fail("File doesn't exist"),
                ErrorKind::PermissionDenied => fail("Insufficient permissions, make sure you have access to the file."),
                _ => fail("Unknown error while opening file")
            }
        }
    }

    println!("=> Checking hash...");
    let mut hash = Sha1::from(&file_data).hexdigest();
    if !versions.contains_key(hash.as_str()) {
        fail("Unknown version. Either your version is not supported or the file has been modified. Open an issue on GitHub.")
    }
    let current_version = versions.get(hash.as_str()).unwrap();
    println!("=> Hash OK. Detected version: {}", current_version.name);

    for (idx, patch) in current_version.patches.iter().enumerate() {
        println!("{color_yellow}=> Patching {} of {}...{color_reset}", idx+1, current_version.patches.len());
        file_data.splice(patch.offset..patch.offset+patch.data.len(), patch.data.clone());
    }

    println!("=> Checking hash...");
    hash = Sha1::from(&file_data).hexdigest();
    if current_version.final_hexdigest != hash.as_str() {
        fail("Unknown error. Open an issue on GitHub.");
    }

    println!("=> Hash OK.");
    println!("=> Writing patched file to {}...", new_filename);

    let output_file = OpenOptions::new().create_new(true).write(true).open(new_filename);

    match output_file {
        Ok(mut file) => write_file(&mut file, &mut file_data),
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => fail("File already exists."),
                ErrorKind::PermissionDenied => fail("Insufficient permissions, make sure you have access to the file."),
                _ => fail("Unknown I/O Error")
            }
        }
    }
    println!("{color_green}Patcher done.{color_reset}");

}
