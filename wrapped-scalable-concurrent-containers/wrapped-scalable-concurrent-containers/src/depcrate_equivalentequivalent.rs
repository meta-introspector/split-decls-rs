// Generated macro for Equivalent (trait)
macro_rules! Depcrate_equivalentEquivalent {
() => {
// Module: crate::equivalent
// Provides: {"Equivalent"}
// Dependencies: {}
# [doc = " Key equivalence trait."] # [doc = ""] # [doc = " [`Hash`](std::hash::Hash) must be implemented to ensure that the same hash value"] # [doc = " is generated for equivalent keys."] pub trait Equivalent < K : ? Sized > { # [doc = " Compares `self` with `key` and returns `true` if they are equal."] fn equivalent (& self , key : & K) -> bool ; }
};
}
