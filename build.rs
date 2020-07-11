use std::process::{Command, Stdio};
use std::fs::{File};
use std::io::{Write};
use std::path::{Path, PathBuf};
use std::env::{self};
use pkg_config::{self, Library};

fn main() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    let slang_lib = pkg_config::Config::new()
        .atleast_version("1.2")
        .probe("slang")
        .unwrap();
    generate_version_rs(&slang_lib);
    println!("cargo:rustc-link-lib=slang");
}

fn generate_version_rs(lib: &Library) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let c_file = Path::new(&out_dir).join("version.c");
    let bin_file = Path::new(&out_dir).join("version");
    let rs_file = Path::new(&out_dir).join("version.rs");

    {
        let c_file_display = c_file.display();
        let mut c_file = File::create(&c_file).expect(&format!("cannot create {}", c_file_display));
        c_file.write_all(br##"
#include <stdio.h>
#include <slang.h>

int main() {
    printf("pub const SLANG_VERSION: u32 = %d;\n", SLANG_VERSION);
    printf("pub const SLANG_VERSION_STRING: &[u8] = b\"%s\\0\";\n", SLANG_VERSION_STRING);
    return 0;
}
"##).expect(&format!("cannot write {}", c_file_display));
    }

    let mut build = cc::Build::new();
    for include_path in lib.include_paths.iter() {
        build.include(include_path);
    }
    let mut compiler = build.try_get_compiler().unwrap().to_command();
    compiler.arg("-o").arg(&bin_file).arg(&c_file);
    let compiler_status = compiler.stdin(Stdio::null()).status().expect(&format!("cannot compile {}", c_file.display()));
    if !compiler_status.success() {
        panic!("{} compilation failed with non-zero {}", c_file.display(), compiler_status);
    }
    let rs_file = File::create(&rs_file).expect(&format!("cannot create {}", rs_file.display()));
    Command::new(&bin_file).stdin(Stdio::null()).stdout(rs_file).status().expect(&format!("{} failed", bin_file.display()));
}
