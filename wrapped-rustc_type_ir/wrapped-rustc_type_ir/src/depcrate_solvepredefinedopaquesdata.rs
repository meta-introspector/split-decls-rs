// Generated macro for PredefinedOpaquesData (struct)
macro_rules! Depcrate_solvePredefinedOpaquesData {
() => {
// Module: crate::solve
// Provides: {"PredefinedOpaquesData"}
// Dependencies: {}
# [doc = " Opaques that are defined in the inference context before a query is called."] # [derive_where (Clone , Hash , PartialEq , Debug , Default ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct PredefinedOpaquesData < I : Interner > { pub opaque_types : Vec < (ty :: OpaqueTypeKey < I > , I :: Ty) > , }
};
}
