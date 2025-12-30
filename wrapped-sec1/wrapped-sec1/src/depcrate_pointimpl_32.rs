// Generated macro for impl_32 (impl)
macro_rules! Depcrate_pointimpl_32 {
() => {
// Module: crate::point
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , Size > Deserialize < 'de > for EncodedPoint < Size > where Size : ModulusSize , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let bytes = serdect :: slice :: deserialize_hex_or_bin_vec (deserializer) ? ; Self :: from_bytes (bytes) . map_err (de :: Error :: custom) } }
};
}
