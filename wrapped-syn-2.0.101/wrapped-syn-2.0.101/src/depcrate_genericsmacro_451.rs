// Generated macro for macro_451 (macro)
macro_rules! Depcrate_genericsmacro_451 {
() => {
// Module: crate::generics
// Provides: {"macro_451"}
// Dependencies: {}
ast_struct ! { # [doc = " A lifetime predicate in a `where` clause: `'a: 'b + 'c`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct PredicateLifetime { pub lifetime : Lifetime , pub colon_token : Token ! [:] , pub bounds : Punctuated < Lifetime , Token ! [+] >, } }
};
}
