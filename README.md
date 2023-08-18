# Cross Contract Calls

## Task Description

Given:

* Three smart contracts: `Contract A`, `Contract B`, `Contract C`.
* Each contract has its own on-chain persistent storage.
* VM which can execute smart contracts on-chain.

How would you design cross-contract calls: `Contract A -> Contract B -> Contract C`?

Note, when the "Contract A -> Contract B -> Contract C" chain execution is finished, all contracts' persistent storage should be updated.

What limitations (disadvantages) do you see in your approach?

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
