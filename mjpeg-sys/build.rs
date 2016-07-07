extern crate gcc;

fn main() {
    gcc::Config::new()
      .file("src/mjpeg_mem.c")
      .file("../quirc-sys/quirc/demo/mjpeg.c")
      .include("../quirc-sys/quirc/demo")
      .compile("libmjpeg.a");
    println!("cargo:rustc-link-lib=jpeg");
}
