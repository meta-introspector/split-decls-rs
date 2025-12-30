// Generated macro for impl_40 (impl)
macro_rules! Depcrate_providerimpl_40 {
() => {
// Module: crate::provider
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "serde"))] impl serde :: Serialize for VariantOffsetsWithMetazoneMembershipKind { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . to_unaligned () . serialize (serializer) } }
};
}
