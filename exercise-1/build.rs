use std::env;
use std::process::Command;

fn main(){
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");

    let cwd= env::current_dir().unwrap().to_string_lossy().to_string();
    let xpdf_dir = format!("{}/xpdf", cwd);
    println!("{}", xpdf_dir);

    Command::new("make")
    .arg("clean")
    .current_dir(xpdf_dir.clone())
    .status()
    .expect("Couldn't clean xpdf directory");

    Command::new("rm")
    .arg("-r")
    .arg("-v")
    .arg("-f")
    .arg(&format!("{}/install", xpdf_dir))
    .current_dir(xpdf_dir.clone())
    .status()
    .expect("Couldn't clean xpdf instal directory");  

    env::set_var("LLVM_CONFIG", "llvm-config-12");

    Command::new("./configure")
    .arg(&format!("--prefix={}/install", xpdf_dir))
    .env("CC", "/usr/local/bin/afl-clang-fast")
    .env("CXX", "/usr/local/bin/afl-clang-fast++")
    .current_dir(xpdf_dir.clone())
    .status()
    .expect("Couldn't configure xpdf");

    Command::new("make")
    .current_dir(xpdf_dir.clone())
    .status()
    .expect("couldn't make xpdf");

    Command::new("make")
    .arg("install")
    .current_dir(xpdf_dir)
    .status()
    .expect("couldn't install xpdf");
    
}