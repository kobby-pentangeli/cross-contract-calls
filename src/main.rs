mod contract_a;
mod contract_b;
mod contract_c;

use wasmer::{imports, Instance, Module, Store, Value};

pub type ExecutionResult<T> = std::result::Result<T, ()>;

fn main() {
    let mut store = Store::default();

    // Load the compiled WASM bytes for each contract.
    // let contract_a_code =
    //     std::fs::read("../target/release/contract_a.wasm").expect("Failed to read contract A wasm");
    // let contract_b_code =
    //     std::fs::read("../target/release/contract_b.wasm").expect("Failed to read contract B wasm");
    // let contract_c_code =
    //     std::fs::read("../target/release/contract_c.wasm").expect("Failed to read contract C wasm");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let profile = std::env::var("CARGO_BUILD_PROFILE").unwrap_or_else(|_| "release".to_string());

    let contract_a_path = format!("{}/target/{}/contract_a.wasm", manifest_dir, profile);
    let contract_a_code = std::fs::read(&contract_a_path).expect("Failed to read contract A wasm");

    let contract_b_path = format!("{}/target/{}/contract_b.wasm", manifest_dir, profile);
    let contract_b_code = std::fs::read(&contract_b_path).expect("Failed to read contract B wasm");

    let contract_c_path = format!("{}/target/{}/contract_c.wasm", manifest_dir, profile);
    let contract_c_code = std::fs::read(&contract_c_path).expect("Failed to read contract C wasm");

    // Create instances for each contract.
    let module_a = Module::new(&store, &contract_a_code).expect("Failed to create module A");
    let module_b = Module::new(&store, &contract_b_code).expect("Failed to create module B");
    let module_c = Module::new(&store, &contract_c_code).expect("Failed to create module C");

    let instance_c =
        Instance::new(&mut store, &module_c, &imports! {}).expect("Failed to instantiate C");
    let execute_contract_c = instance_c
        .exports
        .get_function("execute_contract_c")
        .expect("Function for `Contract C` not found");

    let imports_b = imports! {
        "env" => {
            "execute_contract_c" => execute_contract_c.clone(),
        },
    };
    let instance_b =
        Instance::new(&mut store, &module_b, &imports_b).expect("Failed to instantiate B");
    let execute_contract_b = instance_b
        .exports
        .get_function("execute_contract_b")
        .expect("Function for `Contract B` not found");

    let imports_a = imports! {
        "env" => {
            "execute_contract_b" => execute_contract_b.clone(),
        },
    };
    let instance_a =
        Instance::new(&mut store, &module_a, &imports_a).expect("Failed to instantiate A");
    let execute_chained_calls = instance_a
        .exports
        .get_function("execute_chained_calls")
        .expect("Function for `Contract A` not found");

    // Execute the function
    match execute_chained_calls.call(&mut store, &[]) {
        Ok(results) => {
            if let Some(Value::I32(val)) = results.get(0) {
                println!("Result of chained execution: {}", val);
            } else {
                println!("Unexpected return type");
            }
        }
        Err(e) => {
            println!("Failed to execute chained calls: {:?}", e);
        }
    }
}
