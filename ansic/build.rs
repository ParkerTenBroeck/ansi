extern crate cbindgen;

use std::env;
use std::path::PathBuf;

pub fn main() {
    let crate_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined"),
    );

    let c_config = cbindgen::Config::from_file("cbindgen_c.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    cbindgen::generate_with_config(&crate_dir, c_config)
        .expect("Unable to generate bindings")
        .write_to_file(crate_dir.join("ansic.h"));

    let cpp_config = cbindgen::Config::from_file("cbindgen_cpp.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    cbindgen::generate_with_config(&crate_dir, cpp_config)
        .expect("Unable to generate bindings")
        .write_to_file(crate_dir.join("ansic.hpp"));
}
