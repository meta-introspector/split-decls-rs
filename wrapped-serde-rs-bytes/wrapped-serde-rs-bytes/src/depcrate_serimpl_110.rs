// Generated macro for impl_110 (impl)
macro_rules! Depcrate_serimpl_110 {
() => {
// Module: crate::ser
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a > Serialize for Cow < 'a , [u8] > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
