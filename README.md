# Learning Rust Roadmap



## Rust for Python Programers

### Part I - Foundations
#### Setup Types And Control Flow (Chapters 1-4)
**Chapter 2: Getting Started**

*Creating Rust Projects:*
```rust
cargo new project_name
cd project_name
```

Rust Differences From Python
- fn main() is the entry point
- Semicolons end statements
- Compiled language (cargo build, then run)
- Most errors at compile time

1. Variables and Mutability
    - let & mut: Variables are immutable by default (let) you must explicitly add mut to make them mutable (unlike Python, where variables are always mutable)
    - const & static: Used for compile-time constants (const) and global variables (static) with strict compiler enforcement

2. Ownership, Borrowing, and Types
    - Ownership & References (&, &mut, move): Rust’s unique memory-safety system replaces Python's garbage collection using read-only borrows (&), mutable borrows (&mut), and ownership transfers (move)

    - Type Definitions (struct, enum, impl, trait, type): Define data structures (similar to Python data classes), powerful data-carrying enums, method implementations (impl), interface protocols (trait), and type aliases

3. Control Flow and Visibility
    - Control Flow (match, if let, loop): Provides exhaustive pattern matching (match), conditional destructuring (if let), and looping structures (loop, for, while)

    - Visibility (pub, pub(crate)): Enforces strict compiler-level module privacy and public access, replacing Python's underscore-based naming conventions



### Part II - Core Concepts


### Part III - Advanced Topics & Migration


### Capstone