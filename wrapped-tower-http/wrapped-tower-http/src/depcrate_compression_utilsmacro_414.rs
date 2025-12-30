// Generated macro for macro_414 (macro)
macro_rules! Depcrate_compression_utilsmacro_414 {
() => {
// Module: crate::compression_utils
// Provides: {"macro_414"}
// Dependencies: {}
pin_project ! { # [doc = " `Body` that has been decorated by an `AsyncRead`"] pub (crate) struct WrapBody < M : DecorateAsyncRead > { # [pin] pub read : M :: Output , buf : BytesMut , read_all_data : bool , } }
};
}
