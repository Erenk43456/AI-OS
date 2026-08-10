use std::env;
use std::path::PathBuf;

fn main() {
    let kernel_path = env::var_os("CARGO_BIN_FILE_KERNEL").expect("Kernel binary bulunamadı.");

    let kernel_path = PathBuf::from(kernel_path);

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR bulunamadı."));

    let uefi_image = out_dir.join("ai-os-uefi.img");

    println!("cargo:rerun-if-changed=kernel/src");

    bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_image)
        .expect("UEFI disk image oluşturulamadı.");

    println!("cargo:rustc-env=UEFI_PATH={}", uefi_image.display());
}
