// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bodytimpl_13 {
() => {
// Module: crate::bodyt
// Provides: {"impl_13"}
// Dependencies: {}
impl From < Bytes > for Body { fn from (b : Bytes) -> Self { Body (http_body_util :: Full :: new (b) . map_err (crate :: Error :: new) . boxed () ,) } }
};
}
