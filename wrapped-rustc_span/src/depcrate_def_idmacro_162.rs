// Generated macro for macro_162 (macro)
macro_rules! Depcrate_def_idmacro_162 {
() => {
// Module: crate::def_id
// Provides: {"macro_162"}
// Dependencies: {}
rustc_index :: newtype_index ! { # [doc = " A DefIndex is an index into the hir-map for a crate, identifying a"] # [doc = " particular definition. It should really be considered an interned"] # [doc = " shorthand for a particular DefPath."] # [orderable] # [debug_format = "DefIndex({})"] pub struct DefIndex { # [doc = " The crate root is always assigned index 0 by the AST Map code,"] # [doc = " thanks to `NodeCollector::new`."] const CRATE_DEF_INDEX = 0 ; } }
};
}
