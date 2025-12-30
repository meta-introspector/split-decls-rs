// Generated macro for value (macro)
macro_rules! Depcrate_macrosvalue {
() => {
// Module: crate::macros
// Provides: {"value"}
// Dependencies: {}
# [doc = " Construct a `ConstValue`."] # [macro_export] macro_rules ! value { ($ ($ json : tt) +) => { $ crate :: value_internal ! ($ ($ json) +) } ; }
};
}
