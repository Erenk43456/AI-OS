use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
println!();
println!("=================================");
println!("        AI-OS QEMU Launcher");
println!("=================================");
println!();

let uefi_path = PathBuf::from(
    env::var("UEFI_PATH")
        .expect("UEFI_PATH bulunamadı. Önce cargo build çalıştırın.")
);

println!("[OK] UEFI image bulundu:");
println!("{}", uefi_path.display());
println!();

let qemu = r"C:\Program Files\qemu\qemu-system-x86_64.exe";
let firmware = r"C:\Program Files\qemu\share\edk2-x86_64-code.fd";

println!("[OK] QEMU başlatılıyor...");
println!();

let status = Command::new(qemu)
    .args([
        "-drive",
        &format!(
            "if=pflash,format=raw,readonly=on,file={}",
            firmware
        ),
        "-drive",
        &format!(
            "format=raw,file={}",
            uefi_path.display()
        ),
    ])
    .status()
    .expect("QEMU başlatılamadı.");

if !status.success() {
    eprintln!("QEMU hata koduyla kapandı: {:?}", status.code());
}

}