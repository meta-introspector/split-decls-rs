// Generated macro for macro_38 (macro)
macro_rules! Depcrate_datamacro_38 {
() => {
// Module: crate::data
// Provides: {"macro_38"}
// Dependencies: {}
ast_struct ! { # [doc = " A field of a struct or enum variant."] pub struct Field { # [doc = " Name of the field, if any."] # [doc = ""] # [doc = " Fields of tuple structs have no names."] pub ident : Option < Ident >, # [doc = " Visibility of the field."] pub vis : Visibility , # [doc = " Attributes tagged on the field."] pub attrs : Vec < Attribute >, # [doc = " Type of the field."] pub ty : Ty , pub colon_token : Option < tokens :: Colon >, } }
};
}
