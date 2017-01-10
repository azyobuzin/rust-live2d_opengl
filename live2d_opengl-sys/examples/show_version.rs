extern crate live2d_opengl_sys;

use live2d_opengl_sys::*;

fn main() {
    println!(
        "{} ({})",
         getVersionStr().to_str().unwrap(),
         getVersionNo()
    );
}
