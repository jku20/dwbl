use std::path::PathBuf;
use std::process::Stdio;
use std::{fs, process::Command};

fn generate_test_plugin() -> String {
    let yosys_h = env!("OUT_DIR").to_owned() + "/cxxbridge/crate/dwbl-lib/yosys/kernel/yosys.h";
    let binding = env!("OUT_DIR").to_owned() + "/cxxbridge/include/dwbl-lib/src/lib.rs.h";
    return format!(
        "#include \"{yosys_h}\"
#include \"{binding}\"

USING_YOSYS_NAMESPACE

struct Test1Pass : public Pass {{
	Test1Pass() : Pass(\"test1\", \"creating the absval module\") {{}}
	void execute(std::vector<std::string>, RTLIL::Design *design) override
	{{
        build_module(*design);
	}}
}} Test1Pass;
"
    )
    .to_owned();
}

#[test]
fn test_can_access_any_function() {
    let tmpdir = env!("CARGO_TARGET_TMPDIR");
    let so = format!("{tmpdir}/plugin.so");
    let cc = format!("{tmpdir}/plugin.cc");
    fs::write(&cc, generate_test_plugin()).unwrap();

    // Stupid hack to get the <target>/<profile> directory because that's where cxx puts its
    // archive.
    let out_dir = env!("OUT_DIR");
    let target_profile_dir = PathBuf::from(format!("{out_dir}/../../.."))
        .canonicalize()
        .unwrap();
    let src_root = PathBuf::from(format!("{out_dir}/../../../../../dwbl-lib/src"))
        .canonicalize()
        .unwrap();
    let target_profile_dir = target_profile_dir.as_os_str().to_str().unwrap();
    let src_root = src_root.as_os_str().to_str().unwrap();

    let configed = Command::new("yosys-config")
        .arg("--exec")
        .arg("--cxx")
        .arg("--cxxflags")
        .arg(format!("-I{src_root}"))
        .arg("--ldflags")
        .arg("-o")
        .arg(&so)
        .arg("-shared")
        .arg(&cc)
        .arg("--libs")
        .arg(format!("-L{target_profile_dir}"))
        .arg("-ldwbl_lib")
        .status()
        .unwrap();
    assert!(configed.success());

    let output = Command::new("yosys")
        .arg("-m")
        .arg(&so)
        .arg("-p")
        .arg("test1")
        .stdout(Stdio::null())
        .status()
        .unwrap();

    assert!(output.success());
}
