// Generated macro for impl_63 (impl)
macro_rules! Depcrate_sliceimpl_63 {
() => {
// Module: crate::slice
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < const UPPERCASE : bool > Serialize for HexOrBin < UPPERCASE > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if UPPERCASE { serialize_hex_upper_or_bin (self , serializer) } else { serialize_hex_lower_or_bin (self , serializer) } } }
};
}
