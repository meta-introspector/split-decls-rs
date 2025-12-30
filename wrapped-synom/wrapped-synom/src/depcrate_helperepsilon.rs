// Generated macro for epsilon (macro)
macro_rules! Depcrate_helperepsilon {
() => {
// Module: crate::helper
// Provides: {"epsilon"}
// Dependencies: {}
# [doc = " Parses nothing and always succeeds."] # [doc = ""] # [doc = " This can be useful as a fallthrough case in `alt!`."] # [doc = ""] # [doc = " - **Syntax:** `epsilon!()`"] # [doc = " - **Output:** `()`"] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate syn;"] # [doc = " #[macro_use] extern crate synom;"] # [doc = ""] # [doc = " use syn::Mutability;"] # [doc = " use synom::tokens::Mut;"] # [doc = ""] # [doc = " named!(mutability -> Mutability, alt!("] # [doc = "     syn!(Mut) => { Mutability::Mutable }"] # [doc = "     |"] # [doc = "     epsilon!() => { |_| Mutability::Immutable }"] # [doc = " ));"] # [doc = ""] # [doc = " # fn main() {}"] # [macro_export] macro_rules ! epsilon { ($ i : expr ,) => { :: std :: result :: Result :: Ok (($ i , ())) } ; }
};
}
