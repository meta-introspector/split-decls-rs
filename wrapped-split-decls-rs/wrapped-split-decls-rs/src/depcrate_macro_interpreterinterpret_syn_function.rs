// Generated macro for interpret_syn_function (macro)
macro_rules! Depcrate_macro_interpreterinterpret_syn_function {
() => {
// Module: crate::macro_interpreter
// Provides: {"interpret_syn_function"}
// Dependencies: {}
# [macro_export] macro_rules ! interpret_syn_function { ($ rdf_state : expr , $ func_name : expr , $ wrap_path : expr) => { interpret_wrapped_decl ! ($ rdf_state , $ func_name , $ wrap_path , { let decl_content = std :: fs :: read_to_string ($ wrap_path) . unwrap_or_else (| _ | format ! ("// Wrapped declaration for {}" , $ func_name)) ; $ rdf_state . capture_data ("decl_size" , & decl_content . len () . to_string ()) ; $ rdf_state . capture_data ("decl_type" , "syn_function") ; format ! ("Executed wrapped: {}" , $ func_name) }) } ; }
};
}
