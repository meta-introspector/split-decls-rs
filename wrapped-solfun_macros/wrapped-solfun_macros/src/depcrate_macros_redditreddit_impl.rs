// Generated macro for reddit_impl (function)
macro_rules! Depcrate_macros_redditreddit_impl {
() => {
// Module: crate::macros::reddit
// Provides: {"reddit_impl"}
// Dependencies: {}
# [decl (fn , name = "reddit_impl" , vis = "pub" , hash = "06cb7d7a")] pub fn reddit_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n👽 REDDIT! Conceptual interaction: \"{description}\"\n") ; () } . into () }
};
}
