// Generated macro for wikipedia_impl (function)
macro_rules! Depcrate_macros_wikipediawikipedia_impl {
() => {
// Module: crate::macros::wikipedia
// Provides: {"wikipedia_impl"}
// Dependencies: {}
# [decl (fn , name = "wikipedia_impl" , vis = "pub" , hash = "c91cda37")] pub fn wikipedia_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n📚 WIKIPEDIA! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
};
}
