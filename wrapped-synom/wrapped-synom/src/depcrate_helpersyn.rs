// Generated macro for syn (macro)
macro_rules! Depcrate_helpersyn {
() => {
// Module: crate::helper
// Provides: {"syn"}
// Dependencies: {}
# [doc = " Parse a type through the `Synom` trait."] # [doc = ""] # [doc = " This is a convenience macro used to invoke the `Synom::parse` method for a"] # [doc = " type, you'll find this in quite a few parsers. This is also the primary way"] # [doc = " to parse punctuation."] # [doc = ""] # [doc = " - **Syntax:** `syn!(TYPE)`"] # [doc = " - **Output:** `TYPE`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::Expr;"] # [doc = " use synom::tokens::Dot;"] # [doc = ""] # [doc = " named!(expression -> Expr, syn!(Expr));"] # [doc = ""] # [doc = " named!(expression_dot -> (Expr, Dot), tuple!(syn!(Expr), syn!(Dot)));"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! syn { ($ i : expr , $ t : ty) => { call ! ($ i , <$ t as $ crate :: Synom >:: parse) } ; }
};
}
