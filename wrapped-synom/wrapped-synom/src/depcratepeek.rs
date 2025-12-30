// Generated macro for peek (macro)
macro_rules! Depcratepeek {
() => {
// Module: crate
// Provides: {"peek"}
// Dependencies: {}
# [doc = " Parse a value without consuming it from the input data."] # [doc = ""] # [doc = " - **Syntax:** `peek!(THING)`"] # [doc = " - **Output:** `THING`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::{Expr, Ident};"] # [doc = ""] # [doc = " // Parse an expression that begins with an identifier."] # [doc = " named!(ident_expr -> (Ident, Expr),"] # [doc = "     tuple!(peek!(syn!(Ident)), syn!(Expr))"] # [doc = " );"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! peek { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Ok ((_ , o)) => :: std :: result :: Result :: Ok (($ i , o)) , :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , } } ; ($ i : expr , $ f : expr) => { peek ! ($ i , call ! ($ f)) } ; }
};
}
