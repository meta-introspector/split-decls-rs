// Generated macro for sealed (module)
macro_rules! Depcratesealed {
() => {
// Module: crate
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { # [doc = " This trait being unreachable from outside the crate"] # [doc = " prevents outside implementations of our extension traits."] # [doc = " This allows adding more trait methods in the future."] # [unstable (feature = "sealed" , issue = "none")] pub trait Sealed { } }
};
}
