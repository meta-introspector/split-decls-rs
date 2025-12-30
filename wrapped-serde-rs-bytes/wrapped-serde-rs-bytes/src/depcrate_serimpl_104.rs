// Generated macro for impl_104 (impl)
macro_rules! Depcrate_serimpl_104 {
() => {
// Module: crate::ser
// Provides: {"impl_104"}
// Dependencies: {}
impl Serialize for [u8] { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
