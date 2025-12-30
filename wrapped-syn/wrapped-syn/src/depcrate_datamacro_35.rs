// Generated macro for macro_35 (macro)
macro_rules! Depcrate_datamacro_35 {
() => {
// Module: crate::data
// Provides: {"macro_35"}
// Dependencies: {}
ast_struct ! { # [doc = " An enum variant."] pub struct Variant { # [doc = " Name of the variant."] pub ident : Ident , # [doc = " Attributes tagged on the variant."] pub attrs : Vec < Attribute >, # [doc = " Type of variant."] pub data : VariantData , # [doc = " Explicit discriminant, e.g. `Foo = 1`"] pub discriminant : Option < Expr >, pub eq_token : Option < tokens :: Eq >, } }
};
}
