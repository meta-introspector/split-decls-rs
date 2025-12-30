// Generated macro for Stack (struct)
macro_rules! Depcrate_stackStack {
() => {
// Module: crate::stack
// Provides: {"Stack"}
// Dependencies: {}
# [doc = " Two [`Layer`]s chained together."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tower_layer::{Stack, layer_fn, Layer};"] # [doc = ""] # [doc = " let inner = layer_fn(|service| service+2);"] # [doc = " let outer = layer_fn(|service| service*2);"] # [doc = ""] # [doc = " let inner_outer_stack = Stack::new(inner, outer);"] # [doc = ""] # [doc = " // (4 + 2) * 2 = 12"] # [doc = " // (4 * 2) + 2 = 10"] # [doc = " assert_eq!(inner_outer_stack.layer(4), 12);"] # [doc = " ```"] # [derive (Clone)] pub struct Stack < Inner , Outer > { inner : Inner , outer : Outer , }
};
}
