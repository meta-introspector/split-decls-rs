// Generated macro for impl_105 (impl)
macro_rules! Depcrate_serimpl_105 {
() => {
// Module: crate::ser
// Provides: {"impl_105"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl Serialize for Vec < u8 > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
