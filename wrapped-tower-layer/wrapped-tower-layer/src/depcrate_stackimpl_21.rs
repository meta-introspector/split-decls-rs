// Generated macro for impl_21 (impl)
macro_rules! Depcrate_stackimpl_21 {
() => {
// Module: crate::stack
// Provides: {"impl_21"}
// Dependencies: {}
impl < Inner , Outer > Stack < Inner , Outer > { # [doc = " Creates a new [`Stack`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tower_layer::{Stack, Identity};"] # [doc = ""] # [doc = " let stack = Stack::new(Identity::new(), Identity::new());"] # [doc = " ```"] pub const fn new (inner : Inner , outer : Outer) -> Self { Stack { inner , outer } } }
};
}
