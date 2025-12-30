// Generated macro for macro_449 (macro)
macro_rules! Depcrate_genericsmacro_449 {
() => {
// Module: crate::generics
// Provides: {"macro_449"}
// Dependencies: {}
ast_struct ! { # [doc = " A `where` clause in a definition: `where T: Deserialize<'de>, D:"] # [doc = " 'static`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct WhereClause { pub where_token : Token ! [where] , pub predicates : Punctuated < WherePredicate , Token ! [,] >, } }
};
}
