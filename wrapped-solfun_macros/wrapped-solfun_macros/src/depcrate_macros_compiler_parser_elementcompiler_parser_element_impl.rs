// Generated macro for compiler_parser_element_impl (function)
macro_rules! Depcrate_macros_compiler_parser_elementcompiler_parser_element_impl {
() => {
// Module: crate::macros::compiler_parser_element
// Provides: {"compiler_parser_element_impl"}
// Dependencies: {}
# [decl (fn , name = "compiler_parser_element_impl" , vis = "pub" , hash = "0eef2414")] pub fn compiler_parser_element_impl (input : TokenStream) -> TokenStream { let element_name = parse_macro_input ! (input as LitStr) ; let span = element_name . span () ; quote_spanned ! { span => eprintln ! ("\n🔍 COMPILER PARSER ELEMENT! Interacting with: {}\n" , # element_name) ; () } . into () }
};
}
