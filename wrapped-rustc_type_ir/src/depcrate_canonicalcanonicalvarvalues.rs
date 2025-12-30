// Generated macro for CanonicalVarValues (struct)
macro_rules! Depcrate_canonicalCanonicalVarValues {
() => {
// Module: crate::canonical
// Provides: {"CanonicalVarValues"}
// Dependencies: {}
# [doc = " A set of values corresponding to the canonical variables from some"] # [doc = " `Canonical`. You can give these values to"] # [doc = " `canonical_value.instantiate` to instantiate them into the canonical"] # [doc = " value at the right places."] # [doc = ""] # [doc = " When you canonicalize a value `V`, you get back one of these"] # [doc = " vectors with the original values that were replaced by canonical"] # [doc = " variables. You will need to supply it later to instantiate the"] # [doc = " canonicalized query response."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct CanonicalVarValues < I : Interner > { pub var_values : I :: GenericArgs , }
};
}
