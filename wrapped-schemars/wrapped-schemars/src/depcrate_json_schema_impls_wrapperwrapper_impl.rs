// Generated macro for wrapper_impl (macro)
macro_rules! Depcrate_json_schema_impls_wrapperwrapper_impl {
() => {
// Module: crate::json_schema_impls::wrapper
// Provides: {"wrapper_impl"}
// Dependencies: {}
macro_rules ! wrapper_impl { ($ ($ desc : tt) +) => { forward_impl ! (($ ($ desc) + where T : JsonSchema) => T) ; } ; }
};
}
