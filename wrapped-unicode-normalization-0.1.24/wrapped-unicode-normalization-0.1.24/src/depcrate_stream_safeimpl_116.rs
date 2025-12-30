// Generated macro for impl_116 (impl)
macro_rules! Depcrate_stream_safeimpl_116 {
() => {
// Module: crate::stream_safe
// Provides: {"impl_116"}
// Dependencies: {}
impl < I : Iterator < Item = char > > Iterator for StreamSafe < I > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { let next_ch = match self . buffer . take () . or_else (| | self . iter . next ()) { None => return None , Some (c) => c , } ; let d = classify_nonstarters (next_ch) ; if self . nonstarter_count + d . leading_nonstarters > MAX_NONSTARTERS { self . nonstarter_count = 0 ; self . buffer = Some (next_ch) ; return Some (COMBINING_GRAPHEME_JOINER) ; } if d . leading_nonstarters == d . decomposition_len { self . nonstarter_count += d . decomposition_len ; } else { self . nonstarter_count = d . trailing_nonstarters ; } Some (next_ch) } }
};
}
