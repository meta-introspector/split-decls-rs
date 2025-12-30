// Generated macro for AliasTyKind (enum)
macro_rules! Depcrate_ty_kindAliasTyKind {
() => {
// Module: crate::ty_kind
// Provides: {"AliasTyKind"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum AliasTyKind { # [doc = " A projection `<Type as Trait>::AssocType`."] # [doc = " Can get normalized away if monomorphic enough."] Projection , # [doc = " An associated type in an inherent `impl`"] Inherent , # [doc = " An opaque type (usually from `impl Trait` in type aliases or function return types)"] # [doc = " Can only be normalized away in PostAnalysis mode or its defining scope."] Opaque , # [doc = " A type alias that actually checks its trait bounds."] # [doc = " Currently only used if the type alias references opaque types."] # [doc = " Can always be normalized away."] Free , }
};
}
