use std::env;
use std::path::PathBuf;
use std::process::Command;

const SCHEMA_VERSIONS: [&str; 2] = ["v1", "v2"];

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by cargo"));

    for version in SCHEMA_VERSIONS {
        let schema = format!("schemas/{version}/battery_state.fbs");
        println!("cargo::rerun-if-changed={schema}");

        let status = Command::new("flatc")
            .args(["--rust", "-o"])
            .arg(out_dir.join(version))
            .arg(&schema)
            .status();
        match status {
            Ok(status) if status.success() => {}
            Ok(_) => println!("cargo::error=flatc failed on {schema}"),
            Err(err) => println!("cargo::error=failed to run flatc: {err}"),
        }
    }
}
