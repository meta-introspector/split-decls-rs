// Generated macro for ProjectionPredicate (struct)
macro_rules! Depcrate_predicateProjectionPredicate {
() => {
// Module: crate::predicate
// Provides: {"ProjectionPredicate"}
// Dependencies: {}
# [doc = " This kind of predicate has no *direct* correspondent in the"] # [doc = " syntax, but it roughly corresponds to the syntactic forms:"] # [doc = ""] # [doc = " 1. `T: TraitRef<..., Item = Type>`"] # [doc = " 2. `<T as TraitRef<...>>::Item == Type` (NYI)"] # [doc = ""] # [doc = " In particular, form #1 is \"desugared\" to the combination of a"] # [doc = " normal trait predicate (`T: TraitRef<...>`) and one of these"] # [doc = " predicates. Form #2 is a broader form in that it also permits"] # [doc = " equality between arbitrary types. Processing an instance of"] # [doc = " Form #2 eventually yields one of these `ProjectionPredicate`"] # [doc = " instances to normalize the LHS."] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct ProjectionPredicate < I : Interner > { pub projection_term : AliasTerm < I > , pub term : I :: Term , }
};
}
