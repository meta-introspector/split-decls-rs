// Generated macro for emit_codepoints (function)
macro_rules! Depcrate_raw_emitteremit_codepoints {
() => {
// Module: crate::raw_emitter
// Provides: {"emit_codepoints"}
// Dependencies: {}
pub fn emit_codepoints (emitter : & mut RawEmitter , ranges : & [Range < u32 >]) { emitter . blank_line () ; let mut bitset = emitter . clone () ; let bitset_ok = bitset . emit_bitset (ranges) . is_ok () ; let mut skiplist = emitter . clone () ; skiplist . emit_skiplist (ranges) ; if bitset_ok && bitset . bytes_used <= skiplist . bytes_used { * emitter = bitset ; emitter . desc = String :: from ("bitset") ; } else { * emitter = skiplist ; emitter . desc = String :: from ("skiplist") ; } }
};
}
