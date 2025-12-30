// Generated macro for Locate (trait)
macro_rules! Depcrate_spanLocate {
() => {
// Module: crate::span
// Provides: {"Locate"}
// Dependencies: {}
# [doc = " Trait for types that can be located in the source code."] # [decl (trait , name = "Locate" , vis = "pub" , hash = "f6c8061d")] pub trait Locate { # [doc = " Returns the start offset."] fn start (& self) -> usize ; # [doc = " Returns the end offset."] fn end (& self) -> usize ; }
};
}
