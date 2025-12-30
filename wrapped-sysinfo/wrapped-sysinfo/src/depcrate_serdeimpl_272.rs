// Generated macro for impl_272 (impl)
macro_rules! Depcrate_serdeimpl_272 {
() => {
// Module: crate::serde
// Provides: {"impl_272"}
// Dependencies: {}
# [cfg (feature = "network")] impl Serialize for crate :: Networks { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self . iter ()) } }
};
}
