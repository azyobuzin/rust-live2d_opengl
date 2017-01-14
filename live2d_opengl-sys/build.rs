extern crate gcc;

use std::env;
use std::fs;
use std::path;

fn main() {
    // https://github.com/alexcrichton/gcc-rs/blob/b1601a6bc2c35169cb566aafeac956b9cec0226c/src/lib.rs#L468-L473
    let crt_static = env::var("CARGO_CFG_TARGET_FEATURE").ok()
        .map_or(false, |x| x.contains("crt-static"));

    let mut lib_path = path::PathBuf::new();

    if let Some(lib_dir) = env::var_os("LIVE2DOPENGL_LIB_DIR") {
        lib_path.push(lib_dir);
    }

    lib_path.push(if crt_static { "live2d_opengl_mt.lib" } else { "live2d_opengl.lib" });

    let mut config = gcc::Config::new();

    if let Some(include_dir) = env::var_os("LIVE2DOPENGL_INCLUDE_DIR") {
        config.include(include_dir);
    }

    for cpp_file in CppFilesIterator::new("wrapper").unwrap() {
        config.file(cpp_file);
    }

    config.cpp(true)
        .object(lib_path)
        .define("_DEBUG", None)
        .compile("libl2dwrapper.a");
}

struct CppFilesIterator {
    current_iter: fs::ReadDir,
    stack: Vec<path::PathBuf>,
}

impl CppFilesIterator {
    fn new<P: AsRef<path::Path>>(path: P) -> std::io::Result<CppFilesIterator> {
        Ok(CppFilesIterator {
            current_iter: fs::read_dir(path)?,
            stack: Vec::new(),
        })
    }
}

impl Iterator for CppFilesIterator {
    type Item = path::PathBuf;

    fn next(&mut self) -> Option<path::PathBuf> {
        match self.current_iter.next() {
            Some(r) => match r {
                Ok(x) => {
                    let p = x.path();
                    if p.is_file() && p.extension() == Some("cpp".as_ref()) {
                        Some(p)
                    } else {
                        if p.is_dir() { self.stack.push(p); }
                        self.next()
                    }
                }
                Err(_) => self.next()
            },
            None => match self.stack.pop() {
                Some(x) => match fs::read_dir(x) {
                    Ok(x) => {
                        self.current_iter = x;
                        self.next()
                    }
                    Err(_) => self.next()
                },
                None => None
            }
        }
    }
}
