// Generated macro for macro_1061 (macro)
macro_rules! Depcrate_tymacro_1061 {
() => {
// Module: crate::ty
// Provides: {"macro_1061"}
// Dependencies: {}
ast_struct ! { # [doc = " A raw pointer type: `*const T` or `*mut T`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypePtr { pub star_token : Token ! [*] , pub const_token : Option < Token ! [const] >, pub mutability : Option < Token ! [mut] >, pub elem : Box < Type >, } }
};
}
