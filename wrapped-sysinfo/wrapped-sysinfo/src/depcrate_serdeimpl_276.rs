// Generated macro for impl_276 (impl)
macro_rules! Depcrate_serdeimpl_276 {
() => {
// Module: crate::serde
// Provides: {"impl_276"}
// Dependencies: {}
# [cfg (feature = "user")] impl Serialize for crate :: Users { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self . iter ()) } }
};
}
