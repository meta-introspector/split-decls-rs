// Generated macro for macro_818 (macro)
macro_rules! Depcrate_pathmacro_818 {
() => {
// Module: crate::path
// Provides: {"macro_818"}
// Dependencies: {}
ast_struct ! { # [doc = " Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,"] # [doc = " V>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct AngleBracketedGenericArguments { pub colon2_token : Option < Token ! [::] >, pub lt_token : Token ! [<] , pub args : Punctuated < GenericArgument , Token ! [,] >, pub gt_token : Token ! [>] , } }
};
}
