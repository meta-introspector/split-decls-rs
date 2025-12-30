// Generated macro for AsId (trait)
macro_rules! Depcrate_spanAsId {
() => {
// Module: crate::span
// Provides: {"AsId"}
// Dependencies: {}
# [doc = " Trait implemented by types which have a span `Id`."] pub trait AsId : crate :: sealed :: Sealed { # [doc = " Returns the `Id` of the span that `self` corresponds to, or `None` if"] # [doc = " this corresponds to a disabled span."] fn as_id (& self) -> Option < & Id > ; }
};
}
