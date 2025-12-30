// Generated macro for parse_tokens (function)
macro_rules! Depcrateparse_tokens {
() => {
// Module: crate
// Provides: {"parse_tokens"}
// Dependencies: {}
# [doc = " Parse a `quote::Tokens` of Rust code into the chosen syn data type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #"] # [doc = " # #[macro_use]"] # [doc = " # extern crate error_chain;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate quote;"] # [doc = ""] # [doc = " use syn::Expr;"] # [doc = " #"] # [doc = " # error_chain! {"] # [doc = " #     foreign_links {"] # [doc = " #         Syn(syn::ParseError);"] # [doc = " #     }"] # [doc = " # }"] # [doc = ""] # [doc = " fn run() -> Result<()> {"] # [doc = "     let code = quote!(assert_eq!(u8::max_value(), 255));"] # [doc = "     let expr = syn::parse_tokens::<Expr>(code)?;"] # [doc = "     println!(\"{:#?}\", expr);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() { run().unwrap() }"] # [doc = " ```"] # [cfg (feature = "parsing")] pub fn parse_tokens < T : Synom > (tokens : quote :: Tokens) -> Result < T , ParseError > { _parse (tokens . into ()) }
};
}
