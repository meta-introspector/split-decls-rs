// Generated macro for map (macro)
macro_rules! Depcratemap {
() => {
// Module: crate
// Provides: {"map"}
// Dependencies: {}
# [doc = " Transform the result of a parser by applying a function or closure."] # [doc = ""] # [doc = " - **Syntax:** `map!(THING, FN)`"] # [doc = " - **Output:** the return type of function FN applied to THING"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::{Expr, ExprIf};"] # [doc = ""] # [doc = " fn get_cond(if_: ExprIf) -> Expr {"] # [doc = "     *if_.cond"] # [doc = " }"] # [doc = ""] # [doc = " // Parses an `if` statement but returns the condition part only."] # [doc = " named!(if_condition -> Expr,"] # [doc = "     map!(syn!(ExprIf), get_cond)"] # [doc = " );"] # [doc = ""] # [doc = " // Or equivalently:"] # [doc = " named!(if_condition2 -> Expr,"] # [doc = "     map!(syn!(ExprIf), |if_| *if_.cond)"] # [doc = " );"] # [doc = " #"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! map { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *) , $ g : expr) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , :: std :: result :: Result :: Ok ((i , o)) => :: std :: result :: Result :: Ok ((i , $ crate :: invoke ($ g , o))) , } } ; ($ i : expr , $ f : expr , $ g : expr) => { map ! ($ i , call ! ($ f) , $ g) } ; }
};
}
