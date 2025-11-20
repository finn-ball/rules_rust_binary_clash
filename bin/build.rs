use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let c_code = r#"
#include <stdio.h>

void app_function() {
    printf("Called app_function from binary\n");
}
"#;

    let c_file = out_dir.join("app_code.c");
    fs::write(&c_file, c_code).unwrap();

    cc::Build::new()
        .file(c_file)
        .compile("collision_lib");
        // .compile("collision_lib_myapp");

    println!("Generated file");
}
