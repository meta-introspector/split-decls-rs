// Generated macro for macro_440 (macro)
macro_rules! Depcrate_genericsmacro_440 {
() => {
// Module: crate::generics
// Provides: {"macro_440"}
// Dependencies: {}
ast_struct ! { # [doc = " A set of bound lifetimes: `for<'a, 'b, 'c>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct BoundLifetimes { pub for_token : Token ! [for] , pub lt_token : Token ! [<] , pub lifetimes : Punctuated < GenericParam , Token ! [,] >, pub gt_token : Token ! [>] , } }
};
}
