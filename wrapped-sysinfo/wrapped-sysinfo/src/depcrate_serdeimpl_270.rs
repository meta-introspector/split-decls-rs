// Generated macro for impl_270 (impl)
macro_rules! Depcrate_serdeimpl_270 {
() => {
// Module: crate::serde
// Provides: {"impl_270"}
// Dependencies: {}
# [cfg (feature = "component")] impl Serialize for crate :: Components { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self . iter ()) } }
};
}
