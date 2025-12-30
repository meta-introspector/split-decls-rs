// Generated macro for tests (module)
macro_rules! Depcrate_buildrs_generator_main_logictests {
() => {
// Module: crate::buildrs_generator::main_logic
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use proc_macro2 :: Span ; use syn :: LitStr ; # [test] fn test_generate_main_logic_token_stream_basic () { let decls_output_dir_lit = LitStr :: new ("/tmp/test_crate/src/decls" , Span :: call_site ()) ; let crate_name_sanitized_lit = LitStr :: new ("test_crate_name" , Span :: call_site ()) ; let token_stream = generate_main_logic_token_stream (& decls_output_dir_lit , & crate_name_sanitized_lit ,) ; let code = token_stream . to_string () ; println ! ("Generated code for main_logic.rs test:\n{}" , code) ; } }
};
}
