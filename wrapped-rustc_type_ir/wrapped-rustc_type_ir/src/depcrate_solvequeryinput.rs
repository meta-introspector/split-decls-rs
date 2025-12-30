// Generated macro for QueryInput (struct)
macro_rules! Depcrate_solveQueryInput {
() => {
// Module: crate::solve
// Provides: {"QueryInput"}
// Dependencies: {}
# [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , Goal < I , P >)] # [derive_where (Copy ; I : Interner , Goal < I , P >: Copy)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct QueryInput < I : Interner , P > { pub goal : Goal < I , P > , pub predefined_opaques_in_body : I :: PredefinedOpaques , }
};
}
