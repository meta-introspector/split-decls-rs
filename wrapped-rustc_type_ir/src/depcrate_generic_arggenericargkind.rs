// Generated macro for GenericArgKind (enum)
macro_rules! Depcrate_generic_argGenericArgKind {
() => {
// Module: crate::generic_arg
// Provides: {"GenericArgKind"}
// Dependencies: {}
# [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum GenericArgKind < I : Interner > { Lifetime (I :: Region) , Type (I :: Ty) , Const (I :: Const) , }
};
}
