use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=cgns");
    println!("cargo:rerun-if-changed=cgnslib.h");
    let bindings = bindgen::Builder::default()
        .header("cgnslib.h")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .size_t_is_usize(true)
        .generate()
        .expect("generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("write bindings.rs");
}
