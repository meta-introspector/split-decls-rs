// Generated macro for macro_464 (macro)
macro_rules! Depcrate_genericsmacro_464 {
() => {
// Module: crate::generics
// Provides: {"macro_464"}
// Dependencies: {}
ast_struct ! { # [doc = " A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct PredicateType { # [doc = " Any lifetimes from a `for` binding"] pub lifetimes : Option < BoundLifetimes >, # [doc = " The type being bounded"] pub bounded_ty : Type , pub colon_token : Token ! [:] , # [doc = " Trait and lifetime bounds (`Clone+Send+'static`)"] pub bounds : Punctuated < TypeParamBound , Token ! [+] >, } }
};
}
