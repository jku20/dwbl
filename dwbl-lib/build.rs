fn main() {
    cxx_build::bridge("src/lib.rs")
        // Yosys requires some environment variables to compile. I don't define them, but I do
        // define the environment variable used to check they are defined.
        //
        // TODO: Define the rest of the environment variables used by Yosys.
        .define("_YOSYS_", None)
        // Add yosys itself as a place to look for headers. This is because cxx builds outside of
        // the yosys directory so the nested include directives in yosys's c++ files break.
        .include("yosys")
        .include("src")
        .file("src/lib.cc")
        .compile("dwbl-lib");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.cc");
    println!("cargo:rerun-if-changed=src/lib.h");
}
