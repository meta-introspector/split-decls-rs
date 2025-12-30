// Generated macro for macro_790 (macro)
macro_rules! Depcrate_patmacro_790 {
() => {
// Module: crate::pat
// Provides: {"macro_790"}
// Dependencies: {}
ast_struct ! { # [doc = " A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatSlice { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub elems : Punctuated < Pat , Token ! [,] >, } }
};
}
