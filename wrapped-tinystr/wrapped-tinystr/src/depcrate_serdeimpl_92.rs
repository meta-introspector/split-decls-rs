// Generated macro for impl_92 (impl)
macro_rules! Depcrate_serdeimpl_92 {
() => {
// Module: crate::serde
// Provides: {"impl_92"}
// Dependencies: {}
impl < const N : usize > Serialize for TinyAsciiStr < N > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { self . deref () . serialize (serializer) } else { let mut seq = serializer . serialize_tuple (N) ? ; for byte in self . all_bytes () { seq . serialize_element (byte) ? ; } seq . end () } } }
};
}
