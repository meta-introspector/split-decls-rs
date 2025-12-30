// Generated macro for macro_413 (macro)
macro_rules! Depcrate_genericsmacro_413 {
() => {
// Module: crate::generics
// Provides: {"macro_413"}
// Dependencies: {}
ast_struct ! { # [doc = " Lifetimes and type parameters attached to a declaration of a function,"] # [doc = " enum, trait, etc."] # [doc = ""] # [doc = " This struct represents two distinct optional syntactic elements,"] # [doc = " [generic parameters] and [where clause]. In some locations of the"] # [doc = " grammar, there may be other tokens in between these two things."] # [doc = ""] # [doc = " [generic parameters]: https://doc.rust-lang.org/stable/reference/items/generics.html#generic-parameters"] # [doc = " [where clause]: https://doc.rust-lang.org/stable/reference/items/generics.html#where-clauses"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Generics { pub lt_token : Option < Token ! [<] >, pub params : Punctuated < GenericParam , Token ! [,] >, pub gt_token : Option < Token ! [>] >, pub where_clause : Option < WhereClause >, } }
};
}
