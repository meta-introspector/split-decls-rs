// Generated macro for Clause (struct)
macro_rules! Depcrate_clauseClause {
() => {
// Module: crate::clause
// Provides: {"Clause"}
// Dependencies: {}
# [doc = " A clause."] # [doc = ""] # [doc = " This is stoed in a [`ClauseAlloc`] and thus must have a representation compatible with slice of"] # [doc = " [`LitIdx`] values."] # [doc = ""] # [doc = " It would be nicer to use a DST struct with two members and `repr(C)`, but while that can be"] # [doc = " declared in stable rust, it's almost impossible to work with."] # [repr (transparent)] pub struct Clause { data : [LitIdx] , }
};
}
