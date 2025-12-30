// Generated macro for UnevaluatedConst (struct)
macro_rules! Depcrate_const_kindUnevaluatedConst {
() => {
// Module: crate::const_kind
// Provides: {"UnevaluatedConst"}
// Dependencies: {}
# [doc = " An unevaluated (potentially generic) constant used in the type-system."] # [derive_where (Clone , Copy , Debug , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct UnevaluatedConst < I : Interner > { pub def : I :: DefId , pub args : I :: GenericArgs , }
};
}
