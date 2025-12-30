// Generated macro for OpaqueTypeKey (struct)
macro_rules! Depcrate_opaque_tyOpaqueTypeKey {
() => {
// Module: crate::opaque_ty
// Provides: {"OpaqueTypeKey"}
// Dependencies: {}
# [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct OpaqueTypeKey < I : Interner > { pub def_id : I :: LocalDefId , pub args : I :: GenericArgs , }
};
}
