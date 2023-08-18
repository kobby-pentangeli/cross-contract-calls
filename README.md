# Cross Contract Calls

## Task Description

Given:

* Three smart contracts: `Contract A`, `Contract B`, `Contract C`.
* Each contract has its own on-chain persistent storage.
* VM which can execute smart contracts on-chain.

How would you design cross-contract calls: `Contract A -> Contract B -> Contract C`?

Note, when the "Contract A -> Contract B -> Contract C" chain execution is finished, all contracts' persistent storage should be updated.

What limitations (disadvantages) do you see in your approach?
