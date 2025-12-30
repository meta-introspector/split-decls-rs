// Generated macro for macro_194 (macro)
macro_rules! Depcrate_datamacro_194 {
() => {
// Module: crate::data
// Provides: {"macro_194"}
// Dependencies: {}
ast_struct ! { # [doc = " An enum variant."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Variant { pub attrs : Vec < Attribute >, # [doc = " Name of the variant."] pub ident : Ident , # [doc = " Content stored in the variant."] pub fields : Fields , # [doc = " Explicit discriminant: `Variant = 1`"] pub discriminant : Option < (Token ! [=] , Expr) >, } }
};
}
