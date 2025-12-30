// Generated macro for memmem_ (function)
macro_rules! Depcrate_streammemmem_ {
() => {
// Module: crate::stream
// Provides: {"memmem_"}
// Dependencies: {}
# [cfg (not (feature = "simd"))] fn memmem_ (slice : & [u8] , literal : & [u8]) -> Option < core :: ops :: Range < usize > > { for i in 0 .. slice . len () { let subslice = & slice [i ..] ; if subslice . starts_with (literal) { let i_end = i + literal . len () ; return Some (i .. i_end) ; } } None }
};
}
