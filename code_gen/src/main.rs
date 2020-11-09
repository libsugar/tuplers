use std::{env, fs};

mod code_gen;
use code_gen::*;

fn main() {
    let cwd = env::current_dir().unwrap();
    println!("[Cwd]: {}", cwd.display());
    let mut path = cwd;
    path.push("tuples");
    path.push("src");
    path.push("gen");
    println!("[Gen Target]: {}", path.display());

    fs::create_dir_all(&path).unwrap();

    code_gen(&path);
}
