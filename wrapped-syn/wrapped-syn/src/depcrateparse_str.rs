// Generated macro for parse_str (function)
macro_rules! Depcrateparse_str {
() => {
// Module: crate
// Provides: {"parse_str"}
// Dependencies: {}
# [doc = " Parse a string of Rust code into the chosen syn data type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #"] # [doc = " # #[macro_use]"] # [doc = " # extern crate error_chain;"] # [doc = ""] # [doc = " use syn::Expr;"] # [doc = " #"] # [doc = " # error_chain! {"] # [doc = " #     foreign_links {"] # [doc = " #         Syn(syn::ParseError);"] # [doc = " #     }"] # [doc = " # }"] # [doc = ""] # [doc = " fn run() -> Result<()> {"] # [doc = "     let code = \"assert_eq!(u8::max_value(), 255)\";"] # [doc = "     let expr = syn::parse_str::<Expr>(code)?;"] # [doc = "     println!(\"{:#?}\", expr);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() { run().unwrap() }"] # [doc = " ```"] # [cfg (feature = "parsing")] pub fn parse_str < T : Synom > (s : & str) -> Result < T , ParseError > { _parse (s . parse () ?) }
};
}
