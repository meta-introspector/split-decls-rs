// Generated macro for impl_106 (impl)
macro_rules! Depcrate_serimpl_106 {
() => {
// Module: crate::ser
// Provides: {"impl_106"}
// Dependencies: {}
impl Serialize for Bytes { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
