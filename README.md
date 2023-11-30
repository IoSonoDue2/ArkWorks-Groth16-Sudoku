## Sudoku Verification using Zero-Knowledge Proofs (Groth16)

This repository contains Rust code that leverages Groth16 zero-knowledge proofs (ZKPs) with the Arkworks library to verify Sudoku solutions. It builds upon the foundation laid by [rdi-berkeley's ZKP course lecture 3 code](https://github.com/rdi-berkeley/zkp-course-lecture3-code.git), extending it with additional constraints and applying the Groth16 scheme for verification, which was not present in the original example.

### Overview

The project demonstrates the implementation of ZKPs for Sudoku solution verification. It uses the Groth16 proving scheme, known for its efficiency in generating and verifying proofs. The implementation is done in Rust, utilizing the powerful Arkworks library for constructing and managing zero-knowledge proof systems.

### Code Structure

```rust
#[derive(Clone)]
struct Circuit<const N: usize> {
    solution : [[u8;N];N],
    puzzle: [[u8;N];N],
}
```
The code is structured to define a Circuit struct, representing the Sudoku puzzle and its solution. It includes functions for serializing the puzzle input and the main logic for setting up the proving and verification environment using Groth16.

### Implementation Details

- **Sudoku Circuit:** The circuit is designed to represent a Sudoku grid. It takes in a puzzle (the Sudoku board with some numbers filled in) and the proposed solution.
- **Groth16 Setup:** The code goes through the Groth16 setup phase, generating proving and verification keys.
- **Proof Generation:** It generates proofs for the correctness of the provided Sudoku solution.
- **Verification:** It verifies the generated proofs against the public inputs (the Sudoku puzzle) using the Groth16 verification algorithm.

### Enhancements Over Original Repository

- **Additional Constraints:** The code includes the implementation of the remaining constraints necessary for the full Sudoku verification.
- **Complete ZKP Flow:** Unlike the base example, this project implements the entire zero-knowledge proof cycle, including proof generation and verification using Groth16.

---

This project aims to provide a practical example of using arkworks zero-knowledge proofs developer tools
