// Generated macro for codegen_impl (function)
macro_rules! Depcrate_macros_codegencodegen_impl {
() => {
// Module: crate::macros::codegen
// Provides: {"codegen_impl"}
// Dependencies: {}
# [decl (fn , name = "codegen_impl" , vis = "pub" , hash = "0dab7d2f")] pub fn codegen_impl (input : TokenStream) -> TokenStream { let code_literal = parse_macro_input ! (input as LitStr) ; let span = code_literal . span () ; quote_spanned ! { span => eprintln ! ("\n🤖 CODOGEN! Generated code:\n```rust\n{}\n```\n" , # code_literal) ; () } . into () }
};
}
