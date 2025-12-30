// Generated macro for compiler_type_check_impl (function)
macro_rules! Depcrate_macros_compiler_type_checkcompiler_type_check_impl {
() => {
// Module: crate::macros::compiler_type_check
// Provides: {"compiler_type_check_impl"}
// Dependencies: {}
# [decl (fn , name = "compiler_type_check_impl" , vis = "pub" , hash = "eb0a188f")] pub fn compiler_type_check_impl (input : TokenStream) -> TokenStream { let code_snippet = parse_macro_input ! (input as LitStr) ; let span = code_snippet . span () ; quote_spanned ! { span => eprintln ! ("\n✅ COMPILER TYPE CHECK! Checking type of: {}\n" , # code_snippet) ; () } . into () }
};
}
