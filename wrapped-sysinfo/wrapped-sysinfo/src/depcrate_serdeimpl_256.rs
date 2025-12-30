// Generated macro for impl_256 (impl)
macro_rules! Depcrate_serdeimpl_256 {
() => {
// Module: crate::serde
// Provides: {"impl_256"}
// Dependencies: {}
# [cfg (feature = "disk")] impl Serialize for crate :: Disks { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self . iter ()) } }
};
}
