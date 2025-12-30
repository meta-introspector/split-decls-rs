// Generated macro for macro_463 (macro)
macro_rules! Depcrate_genericsmacro_463 {
() => {
// Module: crate::generics
// Provides: {"macro_463"}
// Dependencies: {}
ast_struct ! { # [doc = " A lifetime predicate in a `where` clause: `'a: 'b + 'c`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct PredicateLifetime { pub lifetime : Lifetime , pub colon_token : Token ! [:] , pub bounds : Punctuated < Lifetime , Token ! [+] >, } }
};
}
