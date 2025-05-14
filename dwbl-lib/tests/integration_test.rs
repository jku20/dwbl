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
		if (design->has(\"\\\\absval\") != 0)
			log_error(\"%s\", log_from_rust(\"A module with the name absval already exists!\\n\"));

		RTLIL::Module *module = design->addModule(\"\\\\absval\");
		log(\"%s\", log_from_rust(\"A module with the name absval already exists!\\n\"));

		RTLIL::Wire *a = module->addWire(\"\\\\a\", 4);
		a->port_input = true;
		a->port_id = 1;

		RTLIL::Wire *y = module->addWire(\"\\\\y\", 4);
		y->port_output = true;
		y->port_id = 2;

		RTLIL::Wire *a_inv = module->addWire(NEW_ID, 4);
		module->addNeg(NEW_ID, a, a_inv, true);
		module->addMux(NEW_ID, a, a_inv, RTLIL::SigSpec(a, 3), y);

		module->fixup_ports();
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
    let target_profile_dir = target_profile_dir.as_os_str().to_str().unwrap();

    let configed = Command::new("yosys-config")
        .arg("--exec")
        .arg("--cxx")
        .arg("--cxxflags")
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
