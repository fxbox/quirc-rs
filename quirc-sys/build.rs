extern crate gcc;
use std::process::Command;
use std::env;

fn make() {
    let exit_code = Command::new("make")
        .current_dir("quirc")
        .arg(format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .arg("libquirc.so")
        .status().unwrap();

    if !exit_code.success() {
        panic!("Could not build the quirc library.");
    }

    let src_lib = "libquirc.so";
    let dst_lib = "quirc/libquirc.so.1";
    //let dst_lib = format!("{}/{}", env::var("OUT_DIR").unwrap(), "libquirc.so.1");

    let exit_code = Command::new("ln")
        .arg("-sf")
        .arg(src_lib)
        .arg(dst_lib.clone())
        .status().unwrap();
    if !exit_code.success() {
        panic!("Unable to link {} to {}", src_lib, dst_lib);
    }

    let cur_dir:String = std::env::current_dir().unwrap().to_string_lossy().into_owned();
    let src_lib = format!("{}/quirc/libquirc.so", cur_dir);
    let dst_lib = format!("{}/{}", env::var("OUT_DIR").unwrap(), "libquirc.so.1");
    let exit_code = Command::new("ln")
        .arg("-sf")
        .arg(src_lib.clone())
        .arg(dst_lib.clone())
        .status().unwrap();
    if !exit_code.success() {
        panic!("Unable to link {} to {}", src_lib, dst_lib);
    }

}

fn main() {
    //let quirc_build_dir = format!("{}/{}", env::var("OUT_DIR").unwrap(), "quirc");
    make();

    gcc::Config::new()
      .file("quirc/demo/convert.c")
      .include("quirc/demo")
      .compile("libconvert.a");

    let cur_dir:String = std::env::current_dir().unwrap().to_string_lossy().into_owned();
    println!("cargo:rustc-link-lib=quirc");
    println!("cargo:rustc-link-lib=static=convert");
    println!("cargo:rustc-link-search=native={}/quirc", cur_dir);
}
