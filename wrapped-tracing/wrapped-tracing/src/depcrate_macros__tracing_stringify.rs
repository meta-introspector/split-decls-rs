// Generated macro for __tracing_stringify (macro)
macro_rules! Depcrate_macros__tracing_stringify {
() => {
// Module: crate::macros
// Provides: {"__tracing_stringify"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __tracing_stringify { ($ ($ k : ident) .+) => { { const NAME : $ crate :: __macro_support :: FieldName < { $ crate :: __macro_support :: FieldName :: len ($ crate :: __macro_support :: stringify ! ($ ($ k) .+)) } > = $ crate :: __macro_support :: FieldName :: new ($ crate :: __macro_support :: stringify ! ($ ($ k) .+)) ; NAME . as_str () } } ; }
};
}
