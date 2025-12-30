// Generated macro for interpret_wrapped_decl (macro)
macro_rules! Depcrate_macro_interpreterinterpret_wrapped_decl {
() => {
// Module: crate::macro_interpreter
// Provides: {"interpret_wrapped_decl"}
// Dependencies: {}
# [macro_export] macro_rules ! interpret_wrapped_decl { ($ rdf_state : expr , $ func_name : expr , $ wrap_path : expr , $ body : block) => { { $ rdf_state . enter_function ($ func_name) ; $ rdf_state . capture_data ("wrap_path" , $ wrap_path) ; let result = $ body ; $ rdf_state . exit_function ($ func_name) ; result } } ; }
};
}
