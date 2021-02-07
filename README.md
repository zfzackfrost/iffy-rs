# iffy-rs
[![Crates.io](https://img.shields.io/crates/v/iffy)](https://crates.io/crates/iffy)
[![Docs.rs](https://img.shields.io/docsrs/iffy)](https://docs.rs/iffy)
 
Rust proc macro for simulating the ternary operator from C-like languages.

------------------------------------------------------------------------------

This crate defines a macro to imitate the ternary operator found in C-like
languages such as C, C++, Java, etc.  The macro can be used to make more
compact conditional expressions in Rust code.

For example, this code in plain rust:
```rust
let a = 20;
let b = 30;
 
// This is the part we will be able to simplify
let min = if a < b {
    a
} else {
    b
};
 
// Check the result
assert_eq!(min, a);
```
... Can be shortened to the following, with this crate:

```rust
let a = 20;
let b = 30;

// Shortened from the previous example
let min = iffy::i!(a < b ? a : b);

// Check the result
assert_eq!(min, a);
```

