// Generated macro for macro_19 (macro)
macro_rules! Depcrate_attrmacro_19 {
() => {
// Module: crate::attr
// Provides: {"macro_19"}
// Dependencies: {}
ast_struct ! { # [doc = " Doc-comments are promoted to attributes that have `is_sugared_doc` = true"] pub struct Attribute { pub style : AttrStyle , pub pound_token : tokens :: Pound , pub bracket_token : tokens :: Bracket , # [doc = " The path of the attribute."] # [doc = ""] # [doc = " E.g. `derive` in `#[derive(Copy)]`"] # [doc = " E.g. `crate::precondition` in `#[crate::precondition x < 5]`"] pub path : Path , # [doc = " Any tokens after the path."] # [doc = ""] # [doc = " E.g. `( Copy )` in `#[derive(Copy)]`"] # [doc = " E.g. `x < 5` in `#[crate::precondition x < 5]`"] pub tts : Vec < TokenTree >, pub is_sugared_doc : bool , } }
};
}
