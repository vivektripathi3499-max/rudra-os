use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

fn main() {
    println!("Building kernel...");

    // Build kernel
    let status = Command::new("cargo")
        .args(["build", "-p", "kernel", "--target", "x86_64-unknown-none"])
        .status()
        .expect("Failed to build kernel");

    if !status.success() {
        panic!("Kernel build failed");
    }

    // Kernel path
    let kernel_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/x86_64-unknown-none/debug/kernel");

    if !kernel_path.exists() {
        panic!("Kernel binary not found");
    }

    // Disk image path
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target");
    let disk_image = out_dir.join("bootable_rudra.img");

    println!("Creating bootable image...");

    let bootloader = bootloader::UefiBoot::new(&kernel_path);
    bootloader
        .create_disk_image(&disk_image)
        .expect("Failed to create disk image");

    // Firmware paths
    let ovmf_code = "/usr/share/OVMF/OVMF_CODE_4M.fd";
    let vars_template = "/usr/share/OVMF/OVMF_VARS_4M.fd";

    let local_vars = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../firmware/OVMF_VARS_4M.fd");

    fs::create_dir_all("../firmware").unwrap();

    if !local_vars.exists() {
        println!("Copying OVMF vars...");
        fs::copy(vars_template, &local_vars)
            .expect("Failed to copy OVMF vars");
    }

    println!("Starting QEMU...");

    let status = Command::new("qemu-system-x86_64")
        .args([
            "-machine", "q35",
            "-m", "512M",
             
            //IMPORTANT: stop reboot loop
             "-no-reboot",

            // smooth mouse device
            "-device", "qemu-xhci",
            
             // tablet for smooth cursor
             "-device", "usb-tablet",

            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code),
            "-drive", &format!("if=pflash,format=raw,file={}", local_vars.display()),
            "-drive", &format!("format=raw,file={}", disk_image.display()),

            "-serial", "stdio",
        ])
        .status()
        .expect("Failed to start QEMU");

    if !status.success() {
        panic!("QEMU exited with error");
    }
}
