use std::path::PathBuf;

fn main() {
    // Directory where Cargo stores generated build artifacts.
    let out_dir = PathBuf::from(
        std::env::var_os("OUT_DIR").unwrap()
    );

    // Get the compiled kernel executable.
    let kernel = PathBuf::from(
        std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap()
    );

    // Create a UEFI bootable disk image.
    let uefi_path = out_dir.join("uefi.img");

    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .expect("Failed to create UEFI disk image");

    // Make the image path available to our root application.
    println!(
        "cargo:rustc-env=UEFI_PATH={}",
        uefi_path.display()
    );
}
