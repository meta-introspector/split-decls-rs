// Generated macro for emit_whitespace (function)
macro_rules! Depcrate_raw_emitteremit_whitespace {
() => {
// Module: crate::raw_emitter
// Provides: {"emit_whitespace"}
// Dependencies: {}
pub fn emit_whitespace (emitter : & mut RawEmitter , ranges : & [Range < u32 >]) { emitter . blank_line () ; let mut cascading = emitter . clone () ; cascading . emit_cascading_map (ranges) ; * emitter = cascading ; emitter . desc = String :: from ("cascading") ; }
};
}
