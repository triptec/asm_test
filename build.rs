extern crate nasm_rs;

fn main() {
    nasm_rs::compile_library("libmc.a", &["src/mc.asm","src/mc.1.asm"]);
    println!("cargo:rustc-link-lib=static=mc");
}
