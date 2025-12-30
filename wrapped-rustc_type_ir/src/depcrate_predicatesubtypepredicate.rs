// Generated macro for SubtypePredicate (struct)
macro_rules! Depcrate_predicateSubtypePredicate {
() => {
// Module: crate::predicate
// Provides: {"SubtypePredicate"}
// Dependencies: {}
# [doc = " Encodes that `a` must be a subtype of `b`. The `a_is_expected` flag indicates"] # [doc = " whether the `a` type is the type that we should label as \"expected\" when"] # [doc = " presenting user diagnostics."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct SubtypePredicate < I : Interner > { pub a_is_expected : bool , pub a : I :: Ty , pub b : I :: Ty , }
};
}
