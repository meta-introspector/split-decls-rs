// Generated macro for DiffableStrRef (trait)
macro_rules! Depcrate_text_abstractionDiffableStrRef {
() => {
// Module: crate::text::abstraction
// Provides: {"DiffableStrRef"}
// Dependencies: {}
# [doc = " Reference to a [`DiffableStr`]."] # [doc = ""] # [doc = " This type exists because while the library only really provides ways to"] # [doc = " work with `&str` and `&[u8]` there are types that deref into those string"] # [doc = " slices such as `String` and `Vec<u8>`."] # [doc = ""] # [doc = " This trait is used in the library whenever it's nice to be able to pass"] # [doc = " strings of different types in."] # [doc = ""] # [doc = " Requires the `text` feature."] pub trait DiffableStrRef { # [doc = " The type of the resolved [`DiffableStr`]."] type Output : DiffableStr + ? Sized ; # [doc = " Resolves the reference."] fn as_diffable_str (& self) -> & Self :: Output ; }
};
}
