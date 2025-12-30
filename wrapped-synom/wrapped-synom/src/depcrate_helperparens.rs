// Generated macro for parens (macro)
macro_rules! Depcrate_helperparens {
() => {
// Module: crate::helper
// Provides: {"parens"}
// Dependencies: {}
# [doc = " Parse a parenthesized-surrounded subtree."] # [doc = ""] # [doc = " This macro will invoke a sub-parser inside of all tokens contained in"] # [doc = " parenthesis. The sub-parser is required to consume all tokens within the"] # [doc = " parens or else this parser will return an error."] # [doc = ""] # [doc = " - **Syntax:** `parens!(SUBPARSER)`"] # [doc = " - **Output:** `(SUBPARSER_RET, Paren)`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::Expr;"] # [doc = " use synom::tokens::Paren;"] # [doc = ""] # [doc = " named!(expr_paren -> (Expr, Paren), parens!(syn!(Expr)));"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! parens { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { $ crate :: tokens :: Paren :: parse ($ i , | i | $ submac ! (i , $ ($ args) *)) } ; ($ i : expr , $ f : expr) => { parens ! ($ i , call ! ($ f)) ; } ; }
};
}
