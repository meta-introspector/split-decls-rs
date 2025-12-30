// Generated macro for Exclusive (enum)
macro_rules! Depcrate_pointer_invariantExclusive {
() => {
// Module: crate::pointer::invariant
// Provides: {"Exclusive"}
// Dependencies: {}
# [doc = " The `Ptr<'a, T>` adheres to the aliasing rules of a `&'a mut T`."] # [doc = ""] # [doc = " The referent of an exclusively-aliased `Ptr` may not be concurrently"] # [doc = " referenced by any other `Ptr`s or references, and may not be accessed (read"] # [doc = " or written) other than via this `Ptr`."] pub enum Exclusive { }
};
}
