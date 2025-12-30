// Generated macro for mlb_invalid (function)
macro_rules! Depcrate_decoder_stringmlb_invalid {
() => {
// Module: crate::decoder::string
// Provides: {"mlb_invalid"}
// Dependencies: {}
fn mlb_invalid < 'i > (stream : & mut & 'i str) -> & 'i str { let offset = stream . as_bytes () . offset_for (| b | (MLB_UNESCAPED , b'"' , b'\n' , ESCAPE , '\r') . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) }
};
}
