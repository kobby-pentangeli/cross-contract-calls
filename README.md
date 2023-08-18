# Cross Contract Calls

This repository contains a simple example demonstrating cross-contract calls using Rust and the [Wasmer](https://wasmer.io/) runtime for [WebAssembly (WASM)](https://webassembly.org/). It simulates the execution of three on-chain smart contracts, where `Contract A` invokes `Contract B`, which subsequently invokes `Contract C`.

## Overview

Given:

* Three smart contracts: `Contract A`, `Contract B`, `Contract C`.
* Each contract has its own on-chain persistent storage.
* VM which can execute smart contracts on-chain.

A. How would one design a solution to facilitate cross-contract calls: i.e., `Contract A -> Contract B -> Contract C`?

B. Note, when the "Contract A -> Contract B -> Contract C" chained execution is finished, all contracts' persistent storage should be updated.

C. What are the limitations of this design approach?

## A. System Design & Architecture

### 1. Contract Initialization

Each contract is written in Rust and can be found in its respective source file (`src/contract_a.rs`, `src/contract_b.rs`, `src/contract_c.rs`). These contracts, once compiled to WebAssembly (WASM) using our build script, can be executed within a WASM virtual machine.

### 2. Compilation to WebAssembly

The [build.rs](/build.rs) script is responsible for compiling each contract source file into WASM binaries. It makes use of the Rust compiler directly and targets the `wasm32-unknown-unknown` architecture.

### 3. Main Execution Context

The [main.rs](/src/main.rs) file provides the execution context for our cross-contract calls. It initializes the Wasmer runtime, loads the WASM binaries for each contract, and finally triggers the execution chain starting from `Contract A`.

## B. Persistent Storage Update

To ensure that the persistent storage of each contract is updated after their execution, each contract contains an associated on-chain storage. Inside the main function (`src/main.rs`), I first retrieve the initial state of all contract storage. Then, after the chained execution, I retrieve the updated states.

## C. Limitations

* Manual Compilation: The current design uses a `build.rs`` script to manually compile each contract to WASM. This isn't ideal for larger projects with many contracts.

* Static Import Structure: The way imports are currently structured is static, i.e., `Contract A` always knows about `Contract B` and `Contract B` knows about `Contract C`. In a dynamic environment, contracts might need a more flexible mechanism to interact with unknown contracts.

* Unsafe Rust: I've used unsafe Rust code (e.g., `unsafe` blocks, `static mut`). This is a potential source of bugs and vulnerabilities, especially in a context as sensitive as smart contracts.

* Lack of Real On-Chain Persistence: The current setup mimics on-chain persistence using Rust's static variables. In an actual blockchain context, storage updates would involve more complex operations and would have associated costs.

* Error Handling: The current approach lacks comprehensive error handling, especially when dealing with potential failures during WASM execution or contract calls.

* Security: The dynamic linking and execution of contracts need thorough validation to prevent malicious behaviors, something this simple example does not cover.

## Setup and Run

1. Add WASM support to your Rust toolchain (if not already set up)

```bash
rustup target add wasm32-unknown-unknown
```

2. Clone the repository and `cd` into the root

```bash
git clone https://github.com/kobby-pentangeli/cross-contract-calls.git
cd cross-contract-calls
```

3. Run

```bash
cargo run --relase
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
