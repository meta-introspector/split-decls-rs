// Generated macro for impl_108 (impl)
macro_rules! Depcrate_serimpl_108 {
() => {
// Module: crate::ser
// Provides: {"impl_108"}
// Dependencies: {}
impl < const N : usize > Serialize for ByteArray < N > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (& * * self) } }
};
}
