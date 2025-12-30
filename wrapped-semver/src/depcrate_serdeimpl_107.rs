// Generated macro for impl_107 (impl)
macro_rules! Depcrate_serdeimpl_107 {
() => {
// Module: crate::serde
// Provides: {"impl_107"}
// Dependencies: {}
impl Serialize for Version { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
};
}
