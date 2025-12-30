// Generated macro for opt_vec (macro)
macro_rules! Depcrate_helperopt_vec {
() => {
// Module: crate::helper
// Provides: {"opt_vec"}
// Dependencies: {}
# [doc = " Turn a failed parse into an empty vector. The argument parser must itself"] # [doc = " return a vector."] # [doc = ""] # [doc = " This is often more convenient than `option!(...)` when the argument produces"] # [doc = " a vector."] # [doc = ""] # [doc = " - **Syntax:** `opt_vec!(THING)`"] # [doc = " - **Output:** `THING`, which must be `Vec<T>`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::{Lifetime, Ty};"] # [doc = " use syn::delimited::Delimited;"] # [doc = " use syn::tokens::*;"] # [doc = ""] # [doc = " named!(bound_lifetimes -> (Vec<Lifetime>, Ty), tuple!("] # [doc = "     opt_vec!(do_parse!("] # [doc = "         syn!(For) >>"] # [doc = "         syn!(Lt) >>"] # [doc = "         lifetimes: call!(Delimited::<Lifetime, Comma>::parse_terminated) >>"] # [doc = "         syn!(Gt) >>"] # [doc = "         (lifetimes.into_vec())"] # [doc = "     )),"] # [doc = "     syn!(Ty)"] # [doc = " ));"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! opt_vec { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Ok ((i , o)) => :: std :: result :: Result :: Ok ((i , o)) , :: std :: result :: Result :: Err (_) => :: std :: result :: Result :: Ok (($ i , Vec :: new ())) } } ; }
};
}
