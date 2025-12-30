// Generated macro for tap (macro)
macro_rules! Depcrate_helpertap {
() => {
// Module: crate::helper
// Provides: {"tap"}
// Dependencies: {}
# [doc = " Run a parser, binding the result to a name, and then evaluating an"] # [doc = " expression."] # [doc = ""] # [doc = " Discards the result of the expression and parser."] # [doc = ""] # [doc = " - **Syntax:** `tap!(NAME : THING => EXPR)`"] # [doc = " - **Output:** `()`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::{Expr, ExprCall};"] # [doc = " use syn::tokens::RArrow;"] # [doc = ""] # [doc = " named!(expr_with_arrow_call -> Expr, do_parse!("] # [doc = "     mut e: syn!(Expr) >>"] # [doc = "     many0!(tap!(arg: tuple!(syn!(RArrow), syn!(Expr)) => {"] # [doc = "         e = Expr {"] # [doc = "             node: ExprCall {"] # [doc = "                 func: Box::new(e),"] # [doc = "                 args: vec![arg.1].into(),"] # [doc = "                 paren_token: Default::default(),"] # [doc = "             }.into(),"] # [doc = "             attrs: Vec::new(),"] # [doc = "         };"] # [doc = "     })) >>"] # [doc = "     (e)"] # [doc = " ));"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc (hidden)] # [macro_export] macro_rules ! tap { ($ i : expr , $ name : ident : $ submac : ident ! ($ ($ args : tt) *) => $ e : expr) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Ok ((i , o)) => { let $ name = o ; $ e ; :: std :: result :: Result :: Ok ((i , ())) } :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , } } ; ($ i : expr , $ name : ident : $ f : expr => $ e : expr) => { tap ! ($ i , $ name : call ! ($ f) => $ e) ; } ; }
};
}
