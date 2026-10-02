```rust
/// Helper macro for creating Symbol values from string literals.
/// This removes the need for a trailing null character in Soroban SDK.
macro_rules! symbol_short {
    ($s:expr) => {
        Symbol::from_str($s)
    };
}
