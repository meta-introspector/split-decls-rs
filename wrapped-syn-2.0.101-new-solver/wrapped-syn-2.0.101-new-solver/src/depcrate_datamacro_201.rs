// Generated macro for macro_201 (macro)
macro_rules! Depcrate_datamacro_201 {
() => {
// Module: crate::data
// Provides: {"macro_201"}
// Dependencies: {}
ast_struct ! { # [doc = " A field of a struct or enum variant."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Field { pub attrs : Vec < Attribute >, pub vis : Visibility , pub mutability : FieldMutability , # [doc = " Name of the field, if any."] # [doc = ""] # [doc = " Fields of tuple structs have no names."] pub ident : Option < Ident >, pub colon_token : Option < Token ! [:] >, pub ty : Type , } }
};
}
