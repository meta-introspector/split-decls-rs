// Generated macro for memmem2_ (function)
macro_rules! Depcrate_streammemmem2_ {
() => {
// Module: crate::stream
// Provides: {"memmem2_"}
// Dependencies: {}
# [cfg (not (feature = "simd"))] fn memmem2_ (slice : & [u8] , literal : (& [u8] , & [u8])) -> Option < core :: ops :: Range < usize > > { for i in 0 .. slice . len () { let subslice = & slice [i ..] ; if subslice . starts_with (literal . 0) { let i_end = i + literal . 0 . len () ; return Some (i .. i_end) ; } if subslice . starts_with (literal . 1) { let i_end = i + literal . 1 . len () ; return Some (i .. i_end) ; } } None }
};
}
