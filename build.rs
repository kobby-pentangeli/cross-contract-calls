use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Define the contracts and their paths
    let contracts = ["contract_a", "contract_b", "contract_c"];

    // Define the target directory based on `OUT_DIR` environment variable
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR is not defined");
    let target_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    for contract in &contracts {
        // Compile the contracts into wasm
        let status = Command::new("rustc")
            .args(&[
                "--target",
                "wasm32-unknown-unknown",
                "--crate-type=cdylib",
                "--crate-name",
                contract,
                &format!("src/{}.rs", contract),
                "-O", // Optimize
                "-o",
                &format!("{}/{}.wasm", target_path.display(), contract),
            ])
            .status()
            .expect("Failed to compile contract to wasm");

        // Check if the command was successful
        if !status.success() {
            panic!("Failed to compile {}.rs", contract);
        }
    }

    // Re-run the build script when a contract source changes
    for contract in &contracts {
        println!("cargo:rerun-if-changed=src/{}.rs", contract);
    }
}
