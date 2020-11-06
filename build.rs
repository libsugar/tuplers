use std::env;

mod code_gen;
use code_gen::*;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    code_gen(out_dir);

    println!("cargo:rerun-if-changed=build.rs");
}
