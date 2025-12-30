// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : serde :: Serialize > serde :: Serialize for ThinVec < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . collect_seq (self . as_slice ()) } }
};
}
