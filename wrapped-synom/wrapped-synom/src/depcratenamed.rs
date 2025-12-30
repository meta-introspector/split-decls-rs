// Generated macro for named (macro)
macro_rules! Depcratenamed {
() => {
// Module: crate
// Provides: {"named"}
// Dependencies: {}
# [doc = " Define a function from a parser combination."] # [doc = ""] # [doc = " - **Syntax:** `named!(NAME -> TYPE, PARSER)` or `named!(pub NAME -> TYPE, PARSER)`"] # [doc = ""] # [doc = " ```rust"] # [doc = " # extern crate syn;"] # [doc = " # #[macro_use] extern crate synom;"] # [doc = " # use syn::Ty;"] # [doc = " # use synom::delimited::Delimited;"] # [doc = " # use synom::tokens::Comma;"] # [doc = " // One or more Rust types separated by commas."] # [doc = " named!(pub comma_separated_types -> Delimited<Ty, Comma>,"] # [doc = "     call!(Delimited::parse_separated_nonempty)"] # [doc = " );"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export] macro_rules ! named { ($ name : ident -> $ o : ty , $ submac : ident ! ($ ($ args : tt) *)) => { fn $ name (i : $ crate :: Cursor) -> $ crate :: PResult <$ o > { $ submac ! (i , $ ($ args) *) } } ; (pub $ name : ident -> $ o : ty , $ submac : ident ! ($ ($ args : tt) *)) => { pub fn $ name (i : $ crate :: Cursor) -> $ crate :: PResult <$ o > { $ submac ! (i , $ ($ args) *) } } ; ($ name : ident ($ ($ params : tt) *) -> $ o : ty , $ submac : ident ! ($ ($ args : tt) *)) => { fn $ name (i : $ crate :: Cursor , $ ($ params) *) -> $ crate :: PResult <$ o > { $ submac ! (i , $ ($ args) *) } } ; (pub $ name : ident ($ ($ params : tt) *) -> $ o : ty , $ submac : ident ! ($ ($ args : tt) *)) => { pub fn $ name (i : $ crate :: Cursor , $ ($ params) *) -> $ crate :: PResult <$ o > { $ submac ! (i , $ ($ args) *) } } ; }
};
}
