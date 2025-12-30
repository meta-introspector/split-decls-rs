// Generated macro for tuple (macro)
macro_rules! Depcratetuple {
() => {
// Module: crate
// Provides: {"tuple"}
// Dependencies: {}
# [doc = " Run a series of parsers and produce all of the results in a tuple."] # [doc = ""] # [doc = " - **Syntax:** `tuple!(A, B, C, ...)`"] # [doc = " - **Output:** `(A, B, C, ...)`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::Ty;"] # [doc = ""] # [doc = " named!(two_types -> (Ty, Ty), tuple!(syn!(Ty), syn!(Ty)));"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! tuple { ($ i : expr , $ ($ rest : tt) *) => { tuple_parser ! ($ i , () , $ ($ rest) *) } ; }
};
}
