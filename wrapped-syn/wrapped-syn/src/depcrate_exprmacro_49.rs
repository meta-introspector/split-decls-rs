// Generated macro for macro_49 (macro)
macro_rules! Depcrate_exprmacro_49 {
() => {
// Module: crate::expr
// Provides: {"macro_49"}
// Dependencies: {}
# [cfg (feature = "full")] ast_struct ! { # [doc = " A field-value pair in a struct literal."] pub struct FieldValue { # [doc = " Name of the field."] pub ident : Ident , # [doc = " Value of the field."] pub expr : Expr , # [doc = " Whether this is a shorthand field, e.g. `Struct { x }`"] # [doc = " instead of `Struct { x: x }`."] pub is_shorthand : bool , # [doc = " Attributes tagged on the field."] pub attrs : Vec < Attribute >, pub colon_token : Option < tokens :: Colon >, } }
};
}
