// Generated macro for CanonicalQueryInput (struct)
macro_rules! Depcrate_canonicalCanonicalQueryInput {
() => {
// Module: crate::canonical
// Provides: {"CanonicalQueryInput"}
// Dependencies: {}
# [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , V)] # [derive_where (Copy ; I : Interner , V : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct CanonicalQueryInput < I : Interner , V > { pub canonical : Canonical < I , V > , pub typing_mode : TypingMode < I > , }
};
}
