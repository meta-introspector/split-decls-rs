// Generated macro for str (macro)
macro_rules! Depcrate_datastr {
() => {
// Module: crate::data
// Provides: {"str"}
// Dependencies: {}
# [doc = " Declare an expected value from within Rust source"] # [doc = ""] # [doc = " Output type: [`Inline`], see [`IntoData`] for operations"] # [doc = ""] # [doc = " ```"] # [doc = " # use snapbox::str;"] # [doc = " str![[\""] # [doc = "     Foo { value: 92 }"] # [doc = " \"]];"] # [doc = " str![r#\"{\"Foo\": 92}\"#];"] # [doc = " ```"] # [macro_export] macro_rules ! str { [$ data : literal] => { $ crate :: str ! [[$ data]] } ; [[$ data : literal]] => { { let position = $ crate :: data :: Position { file : $ crate :: utils :: current_rs ! () , line : line ! () , column : column ! () , } ; let inline = $ crate :: data :: Inline { position , data : $ data , } ; inline } } ; [] => { $ crate :: str ! [[""]] } ; [[]] => { $ crate :: str ! [[""]] } ; }
};
}
