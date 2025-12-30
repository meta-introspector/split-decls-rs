// Generated macro for impl_24 (impl)
macro_rules! Depcrate_spannedimpl_24 {
() => {
// Module: crate::spanned
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : serde_core :: ser :: Serialize > serde_core :: ser :: Serialize for Spanned < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: ser :: Serializer , { self . value . serialize (serializer) } }
};
}
