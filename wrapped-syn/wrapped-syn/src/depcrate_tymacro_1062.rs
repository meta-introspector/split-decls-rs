// Generated macro for macro_1062 (macro)
macro_rules! Depcrate_tymacro_1062 {
() => {
// Module: crate::ty
// Provides: {"macro_1062"}
// Dependencies: {}
ast_struct ! { # [doc = " A reference type: `&'a T` or `&'a mut T`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeReference { pub and_token : Token ! [&] , pub lifetime : Option < Lifetime >, pub mutability : Option < Token ! [mut] >, pub elem : Box < Type >, } }
};
}
