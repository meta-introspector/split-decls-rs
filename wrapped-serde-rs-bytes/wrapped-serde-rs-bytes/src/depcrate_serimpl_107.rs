// Generated macro for impl_107 (impl)
macro_rules! Depcrate_serimpl_107 {
() => {
// Module: crate::ser
// Provides: {"impl_107"}
// Dependencies: {}
impl < const N : usize > Serialize for [u8 ; N] { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
