use bindgen::Builder;
use std::env::var as env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn rebuild(is_rebuild: bool) {
    let cargo_path_str =
        env("CARGO_MANIFEST_DIR").expect("failed to get CARGO_MANIFEST_DIR from env");
    let cargo_path = PathBuf::from(cargo_path_str);
    let local_path = cargo_path.join(".local");
    let cef_path = local_path.join("cef");

    let wrapper_path_str = cargo_path
        .join("cef-bindings")
        .join("cef-wrapper.h")
        .to_string_lossy()
        .to_string();
    let cef_path_str = cef_path.to_string_lossy();
    let arg = format!("-I{}", cef_path_str);

    let mut bindings = Builder::default()
        .header(wrapper_path_str)
        .clang_arg(arg)
        .derive_default(true)
        .layout_tests(true)
        .generate_comments(true)
        .allowlist_recursively(true)
        .generate()
        .expect("failed to generate bindings")
        .to_string();

    bindings = BINDINGS_HEADER.to_owned() + &bindings;
    bindings = bindings.replacen(&EXTERN_C, &(LINK.to_owned() + EXTERN_C), 1);

    let target = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let target_path = cargo_path.join("target").join(target);
    if !is_rebuild {
        let target_path_str = target_path.to_string_lossy().to_string();
        println!("cargo:rustc-link-search=native={}", target_path_str);
        println!("cargo:rustc-link-lib=dylib=cef");
    }

    let bindings_path = local_path.join("cef-bindings.rs");
    let file = File::create(&bindings_path).expect("failed to create bindings file");

    BufWriter::new(file)
        .write_all(bindings.as_bytes())
        .expect("failed to write bindings file");
}

static BINDINGS_HEADER: &'static str = "#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

";
static LINK: &'static str = r##"
#[link(name = "cef")]
"##;
static EXTERN_C: &'static str = r##"extern "C" {"##;
