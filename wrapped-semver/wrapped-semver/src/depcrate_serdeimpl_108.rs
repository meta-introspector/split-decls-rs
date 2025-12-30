// Generated macro for impl_108 (impl)
macro_rules! Depcrate_serdeimpl_108 {
() => {
// Module: crate::serde
// Provides: {"impl_108"}
// Dependencies: {}
impl Serialize for VersionReq { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
};
}
