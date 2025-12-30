// Generated macro for option (macro)
macro_rules! Depcrate_helperoption {
() => {
// Module: crate::helper
// Provides: {"option"}
// Dependencies: {}
# [doc = " Turn a failed parse into `None` and a successful parse into `Some`."] # [doc = ""] # [doc = " - **Syntax:** `option!(THING)`"] # [doc = " - **Output:** `Option<THING>`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::tokens::Bang;"] # [doc = ""] # [doc = " named!(maybe_bang -> Option<Bang>, option!(syn!(Bang)));"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! option { ($ i : expr , $ submac : ident ! ($ ($ args : tt) *)) => { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Ok ((i , o)) => :: std :: result :: Result :: Ok ((i , Some (o))) , :: std :: result :: Result :: Err (_) => :: std :: result :: Result :: Ok (($ i , None)) , } } ; ($ i : expr , $ f : expr) => { option ! ($ i , call ! ($ f)) ; } ; }
};
}
