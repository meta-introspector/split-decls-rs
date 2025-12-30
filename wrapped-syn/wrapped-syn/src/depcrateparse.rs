// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse tokens of source code into the chosen syn data type."] # [doc = ""] # [doc = " This is preferred over parsing a string because tokens are able to preserve"] # [doc = " information about where in the user's code they were originally written (the"] # [doc = " \"span\" of the token), possibly allowing the compiler to produce better error"] # [doc = " messages."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " extern crate proc_macro;"] # [doc = " use proc_macro::TokenStream;"] # [doc = ""] # [doc = " extern crate syn;"] # [doc = ""] # [doc = " #[macro_use]"] # [doc = " extern crate quote;"] # [doc = ""] # [doc = " use syn::DeriveInput;"] # [doc = ""] # [doc = " #[proc_macro_derive(MyMacro)]"] # [doc = " pub fn my_macro(input: TokenStream) -> TokenStream {"] # [doc = "     // Parse the tokens into a syntax tree"] # [doc = "     let ast: DeriveInput = syn::parse(input).unwrap();"] # [doc = ""] # [doc = "     // Build the output, possibly using quasi-quotation"] # [doc = "     let expanded = quote! {"] # [doc = "         /* ... */"] # [doc = "     };"] # [doc = ""] # [doc = "     // Parse back to a token stream and return it"] # [doc = "     expanded.parse().unwrap()"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "parsing")] pub fn parse < T > (tokens : proc_macro :: TokenStream) -> Result < T , ParseError > where T : Synom , { _parse (tokens . into ()) }
};
}
