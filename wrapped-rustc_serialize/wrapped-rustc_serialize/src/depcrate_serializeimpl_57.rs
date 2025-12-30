// Generated macro for impl_57 (impl)
macro_rules! Depcrate_serializeimpl_57 {
() => {
// Module: crate::serialize
// Provides: {"impl_57"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for path :: PathBuf { fn decode (d : & mut D) -> path :: PathBuf { let bytes : String = Decodable :: decode (d) ; path :: PathBuf :: from (bytes) } }
};
}
