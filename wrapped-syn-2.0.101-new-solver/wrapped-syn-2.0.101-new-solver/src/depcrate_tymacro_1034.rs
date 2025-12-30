// Generated macro for macro_1034 (macro)
macro_rules! Depcrate_tymacro_1034 {
() => {
// Module: crate::ty
// Provides: {"macro_1034"}
// Dependencies: {}
ast_struct ! { # [doc = " A fixed size array type: `[T; n]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeArray { pub bracket_token : token :: Bracket , pub elem : Box < Type >, pub semi_token : Token ! [;] , pub len : Expr , } }
};
}
