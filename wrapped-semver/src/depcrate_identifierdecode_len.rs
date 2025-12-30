// Generated macro for decode_len (function)
macro_rules! Depcrate_identifierdecode_len {
() => {
// Module: crate::identifier
// Provides: {"decode_len"}
// Dependencies: {}
unsafe fn decode_len (ptr : * const u8) -> NonZeroUsize { let [first , second] = unsafe { ptr :: read (ptr as * const [u8 ; 2]) } ; if second < 0x80 { unsafe { NonZeroUsize :: new_unchecked ((first & 0x7f) as usize) } } else { return unsafe { decode_len_cold (ptr) } ; # [cold] # [inline (never)] unsafe fn decode_len_cold (mut ptr : * const u8) -> NonZeroUsize { let mut len = 0 ; let mut shift = 0 ; loop { let byte = unsafe { * ptr } ; if byte < 0x80 { return unsafe { NonZeroUsize :: new_unchecked (len) } ; } ptr = unsafe { ptr . add (1) } ; len += ((byte & 0x7f) as usize) << shift ; shift += 7 ; } } } }
};
}
