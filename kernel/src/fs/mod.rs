// kernel/src/fs/mod.rs

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use crate::println;

pub struct File {
    pub name: String,         
    pub data: String,         
    pub read_only: bool,      
}
pub mod vfs;

// Dynamically sizing vector for unlimited files in RAM
static FILES: Mutex<Vec<File>> = Mutex::new(Vec::new());

pub fn init() {
    println!("[fs] Virtual Filesystem Initialized");

    let mut fs = FILES.lock();
    fs.push(File { 
        name: String::from("kernel.bin"), 
        data: String::from("BINARY_DATA"),
        read_only: true,
    });
    fs.push(File { 
        name: String::from("readme.txt"), 
        data: String::from("Welcome to Rudra OS!"),
        read_only: true,
    });
}

pub fn list_files() {
    let fs = FILES.lock();
    println!("--- Filesystem Contents ---");
    for file in fs.iter() {
        let status = if file.read_only { "[RO]" } else { "[RW]" };
        println!("{} - {}", status, file.name);
    }
}

// 🔥 ADDED: This fixes the E0425 error for create_file
pub fn create_file(name: &str) {
    let mut fs = FILES.lock();

    // Prevent duplicate filenames
    if fs.iter().any(|f| f.name == name) {
        println!("Error: File '{}' already exists", name);
        return;
    }

    fs.push(File {
        name: String::from(name),
        data: String::new(),
        read_only: false,
    });
    
    println!("File '{}' created.", name);
}

// 🔥 ADDED: This fixes the E0425 error for write_file
pub fn write_file(name: &str, content: &str) {
    let mut fs = FILES.lock();

    for file in fs.iter_mut() {
        if file.name == name {
            if file.read_only {
                println!("Error: '{}' is a protected system file.", name);
                return;
            }
            file.data = String::from(content);
            println!("Saved to '{}'", name);
            return;
        }
    }
    println!("Error: File '{}' not found", name);
}

pub fn read_file(name: &str) {
    let fs = FILES.lock();
    for file in fs.iter() {
        if file.name == name {
            println!("--- {} ---\n{}\n----------", file.name, file.data);
            return;
        }
    }
    println!("Error: File '{}' not found", name);
}

pub fn delete_file(name: &str) {
    let mut fs = FILES.lock();
    let original_len = fs.len();
    
    if let Some(file) = fs.iter().find(|f| f.name == name) {
        if file.read_only {
            println!("Error: Cannot delete protected file '{}'", name);
            return;
        }
    }

    fs.retain(|f| f.name != name);

    if fs.len() < original_len {
        println!("File '{}' deleted.", name);
    } else {
        println!("Error: File '{}' not found.", name);
    }
}
