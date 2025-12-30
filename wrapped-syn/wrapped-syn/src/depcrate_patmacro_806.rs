// Generated macro for macro_806 (macro)
macro_rules! Depcrate_patmacro_806 {
() => {
// Module: crate::pat
// Provides: {"macro_806"}
// Dependencies: {}
ast_struct ! { # [doc = " A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatSlice { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub elems : Punctuated < Pat , Token ! [,] >, } }
};
}
