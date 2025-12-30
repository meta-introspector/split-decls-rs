// Generated macro for Comparable (trait)
macro_rules! Depcrate_equivalentComparable {
() => {
// Module: crate::equivalent
// Provides: {"Comparable"}
// Dependencies: {}
# [doc = " Key ordering trait."] pub trait Comparable < K : ? Sized > : Equivalent < K > { # [doc = " Compares `self` with `key` and returns their ordering."] fn compare (& self , key : & K) -> Ordering ; }
};
}
