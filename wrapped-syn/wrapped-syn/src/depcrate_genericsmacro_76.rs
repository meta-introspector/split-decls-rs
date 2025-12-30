// Generated macro for macro_76 (macro)
macro_rules! Depcrate_genericsmacro_76 {
() => {
// Module: crate::generics
// Provides: {"macro_76"}
// Dependencies: {}
ast_struct ! { # [doc = " A lifetime definition, e.g. `'a: 'b+'c+'d`"] pub struct LifetimeDef { pub attrs : Vec < Attribute >, pub lifetime : Lifetime , pub colon_token : Option < tokens :: Colon >, pub bounds : Delimited < Lifetime , tokens :: Add >, } }
};
}
