// Generated macro for impl_64 (impl)
macro_rules! Depcrate_sliceimpl_64 {
() => {
// Module: crate::slice
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'de , const UPPERCASE : bool > Deserialize < 'de > for HexOrBin < UPPERCASE > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserialize_hex_or_bin_vec (deserializer) . map (Self) } }
};
}
