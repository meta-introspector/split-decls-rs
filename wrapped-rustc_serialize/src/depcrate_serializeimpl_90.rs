// Generated macro for impl_90 (impl)
macro_rules! Depcrate_serializeimpl_90 {
() => {
// Module: crate::serialize
// Provides: {"impl_90"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for Hash64 { # [inline] fn decode (d : & mut D) -> Self { Self :: new (u64 :: from_le_bytes (d . read_raw_bytes (8) . try_into () . unwrap ())) } }
};
}
