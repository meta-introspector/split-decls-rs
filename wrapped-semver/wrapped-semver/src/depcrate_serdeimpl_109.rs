// Generated macro for impl_109 (impl)
macro_rules! Depcrate_serdeimpl_109 {
() => {
// Module: crate::serde
// Provides: {"impl_109"}
// Dependencies: {}
impl Serialize for Comparator { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
};
}
