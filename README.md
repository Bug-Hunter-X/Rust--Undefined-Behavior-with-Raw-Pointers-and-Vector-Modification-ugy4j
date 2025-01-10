# Rust: Undefined Behavior with Raw Pointers and Vector Modification

This repository demonstrates a common error in Rust involving the unsafe use of raw pointers with vectors. Modifying a vector's contents through a raw pointer after its length or capacity changes leads to undefined behavior, potentially causing crashes or data corruption.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file offers a corrected version.

This example highlights the importance of careful memory management when working with raw pointers in Rust and emphasizes the benefits of using safe abstractions wherever possible. The unsafe block should be avoided in most cases and only used when absolutely necessary, while strictly adhering to memory safety rules.