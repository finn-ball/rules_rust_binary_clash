use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let c_code = r#"
#include <stdio.h>

void lib_function() {
    printf("Called lib_function from library\n");
}
"#;

    let c_file = out_dir.join("lib_code.c");
    fs::write(&c_file, c_code).unwrap();

    cc::Build::new()
        .file(c_file)
        .compile("collision_lib");
        // .compile("collision_lib_mylib");

    println!("Library build.rs: Generated libcollision_lib_mylib.a");
}
