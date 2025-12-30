// Generated macro for impl_109 (impl)
macro_rules! Depcrate_serimpl_109 {
() => {
// Module: crate::ser
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl Serialize for ByteBuf { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self) } }
};
}
