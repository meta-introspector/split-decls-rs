// Generated macro for terminated (macro)
macro_rules! Depcrateterminated {
() => {
// Module: crate
// Provides: {"terminated"}
// Dependencies: {}
# [doc = " Parse two things, returning the value of the first."] # [doc = ""] # [doc = " - **Syntax:** `terminated!(THING, AFTER)`"] # [doc = " - **Output:** `THING`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::Expr;"] # [doc = " use synom::tokens::Pound;"] # [doc = ""] # [doc = " // An expression terminated by ##."] # [doc = " named!(expr_pound_pound -> Expr,"] # [doc = "     terminated!(syn!(Expr), tuple!(syn!(Pound), syn!(Pound)))"] # [doc = " );"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! terminated { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *) , $ submac2 : ident ! ($ ($ args2 : tt) *)) => { match tuple ! ($ i , $ submac ! ($ ($ args) *) , $ submac2 ! ($ ($ args2) *)) { :: std :: result :: Result :: Ok ((i , (o , _))) => :: std :: result :: Result :: Ok ((i , o)) , :: std :: result :: Result :: Err (err) => :: std :: result :: Result :: Err (err) , } } ; ($ i : expr , $ submac : ident ! ($ ($ args : tt) *) , $ g : expr) => { terminated ! ($ i , $ submac ! ($ ($ args) *) , call ! ($ g)) } ; ($ i : expr , $ f : expr , $ submac : ident ! ($ ($ args : tt) *)) => { terminated ! ($ i , call ! ($ f) , $ submac ! ($ ($ args) *)) } ; ($ i : expr , $ f : expr , $ g : expr) => { terminated ! ($ i , call ! ($ f) , call ! ($ g)) } ; }
};
}
