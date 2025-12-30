// Generated macro for CoroutineWitnessTypes (struct)
macro_rules! Depcrate_ty_kindCoroutineWitnessTypes {
() => {
// Module: crate::ty_kind
// Provides: {"CoroutineWitnessTypes"}
// Dependencies: {}
# [derive_where (Clone , Copy , Debug , PartialEq , Hash ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct CoroutineWitnessTypes < I : Interner > { pub types : I :: Tys , pub assumptions : I :: RegionAssumptions , }
};
}
