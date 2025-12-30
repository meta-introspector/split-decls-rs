// Generated macro for impl_65 (impl)
macro_rules! Depcrate_arcimpl_65 {
() => {
// Module: crate::arc
// Provides: {"impl_65"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : Serialize > Serialize for Arc < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer , { (* * self) . serialize (serializer) } }
};
}
