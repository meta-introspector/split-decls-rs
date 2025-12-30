// Generated macro for macro_462 (macro)
macro_rules! Depcrate_genericsmacro_462 {
() => {
// Module: crate::generics
// Provides: {"macro_462"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " A single predicate in a `where` clause: `T: Deserialize<'de>`."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] # [non_exhaustive] pub enum WherePredicate { # [doc = " A lifetime predicate in a `where` clause: `'a: 'b + 'c`."] Lifetime (PredicateLifetime) , # [doc = " A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`."] Type (PredicateType) , } }
};
}
