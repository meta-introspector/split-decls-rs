// Generated macro for memmem (function)
macro_rules! Depcrate_streammemmem {
() => {
// Module: crate::stream
// Provides: {"memmem"}
// Dependencies: {}
# [inline (always)] fn memmem (slice : & [u8] , literal : & [u8]) -> Option < core :: ops :: Range < usize > > { match literal . len () { 0 => Some (0 .. 0) , 1 => memchr (literal [0] , slice) . map (| i | i .. i + 1) , _ => memmem_ (slice , literal) , } }
};
}
