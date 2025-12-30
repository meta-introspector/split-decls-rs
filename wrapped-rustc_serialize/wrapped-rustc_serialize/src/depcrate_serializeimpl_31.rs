// Generated macro for impl_31 (impl)
macro_rules! Depcrate_serializeimpl_31 {
() => {
// Module: crate::serialize
// Provides: {"impl_31"}
// Dependencies: {}
impl < D : Decoder > Decodable < D > for String { fn decode (d : & mut D) -> String { d . read_str () . to_owned () } }
};
}
