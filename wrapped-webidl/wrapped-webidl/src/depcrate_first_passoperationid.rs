// Generated macro for OperationId (enum)
macro_rules! Depcrate_first_passOperationId {
() => {
// Module: crate::first_pass
// Provides: {"OperationId"}
// Dependencies: {}
# [derive (PartialEq , Eq , PartialOrd , Ord , Debug , Clone , Copy)] pub (crate) enum OperationId < 'src > { # [doc = " The name of a constructor in crates/web-sys/webidls/enabled/*.webidl"] # [doc = ""] # [doc = " ex: Constructor(Some(\"ImageData\"))"] Constructor (Option < & 'src str >) , NamedConstructor (IgnoreTraits < & 'src str >) , # [doc = " The name of a function in crates/web-sys/webidls/enabled/*.webidl"] # [doc = ""] # [doc = " ex: Operation(Some(\"vertexAttrib1fv\"))"] Operation (Option < & 'src str >) , IndexingGetter , IndexingSetter , IndexingDeleter , }
};
}
