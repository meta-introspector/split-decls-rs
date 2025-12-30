// Generated macro for huggingface_impl (function)
macro_rules! Depcrate_macros_huggingfacehuggingface_impl {
() => {
// Module: crate::macros::huggingface
// Provides: {"huggingface_impl"}
// Dependencies: {}
# [decl (fn , name = "huggingface_impl" , vis = "pub" , hash = "5ed700cf")] pub fn huggingface_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🤗 HUGGINGFACE! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
};
}
