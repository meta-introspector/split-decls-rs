// Generated macro for macro_58 (macro)
macro_rules! Depcrate_exprmacro_58 {
() => {
// Module: crate::expr
// Provides: {"macro_58"}
// Dependencies: {}
# [cfg (feature = "full")] ast_struct ! { # [doc = " A single field in a struct pattern"] # [doc = ""] # [doc = " Patterns like the fields of Foo `{ x, ref y, ref mut z }`"] # [doc = " are treated the same as `x: x, y: ref y, z: ref mut z`,"] # [doc = " except `is_shorthand` is true"] pub struct FieldPat { # [doc = " The identifier for the field"] pub ident : Ident , # [doc = " The pattern the field is destructured to"] pub pat : Box < Pat >, pub is_shorthand : bool , pub colon_token : Option < tokens :: Colon >, pub attrs : Vec < Attribute >, } }
};
}
