// Generated macro for impl_91 (impl)
macro_rules! Depcrate_serializeimpl_91 {
() => {
// Module: crate::serialize
// Provides: {"impl_91"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for Hash128 { # [inline] fn decode (d : & mut D) -> Self { Self :: new (u128 :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ())) } }
};
}
