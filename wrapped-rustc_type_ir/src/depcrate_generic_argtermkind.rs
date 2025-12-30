// Generated macro for TermKind (enum)
macro_rules! Depcrate_generic_argTermKind {
() => {
// Module: crate::generic_arg
// Provides: {"TermKind"}
// Dependencies: {}
# [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum TermKind < I : Interner > { Ty (I :: Ty) , Const (I :: Const) , }
};
}
