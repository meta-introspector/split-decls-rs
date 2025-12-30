// Generated macro for macro_84 (macro)
macro_rules! Depcrate_genericsmacro_84 {
() => {
// Module: crate::generics
// Provides: {"macro_84"}
// Dependencies: {}
ast_enum_of_structs ! { # [doc = " A single predicate in a `where` clause"] pub enum WherePredicate { # [doc = " A type binding, e.g. `for<'c> Foo: Send+Clone+'c`"] pub BoundPredicate (WhereBoundPredicate { # [doc = " Any lifetimes from a `for` binding"] pub bound_lifetimes : Option < BoundLifetimes >, # [doc = " The type being bounded"] pub bounded_ty : Ty , pub colon_token : tokens :: Colon , # [doc = " Trait and lifetime bounds (`Clone+Send+'static`)"] pub bounds : Delimited < TyParamBound , tokens :: Add >, }) , # [doc = " A lifetime predicate, e.g. `'a: 'b+'c`"] pub RegionPredicate (WhereRegionPredicate { pub lifetime : Lifetime , pub colon_token : Option < tokens :: Colon >, pub bounds : Delimited < Lifetime , tokens :: Add >, }) , # [doc = " An equality predicate (unsupported)"] pub EqPredicate (WhereEqPredicate { pub lhs_ty : Ty , pub eq_token : tokens :: Eq , pub rhs_ty : Ty , }) , } }
};
}
