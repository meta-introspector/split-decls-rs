// Generated macro for basic_invalid (function)
macro_rules! Depcrate_decoder_stringbasic_invalid {
() => {
// Module: crate::decoder::string
// Provides: {"basic_invalid"}
// Dependencies: {}
fn basic_invalid < 'i > (stream : & mut & 'i str) -> & 'i str { let offset = stream . as_bytes () . offset_for (| b | (BASIC_UNESCAPED , ESCAPE) . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) }
};
}
