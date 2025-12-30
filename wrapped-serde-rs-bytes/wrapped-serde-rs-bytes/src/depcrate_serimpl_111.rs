// Generated macro for impl_111 (impl)
macro_rules! Depcrate_serimpl_111 {
() => {
// Module: crate::ser
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a > Serialize for Cow < 'a , Bytes > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
