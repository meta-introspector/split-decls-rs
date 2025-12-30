// Generated macro for impl_24 (impl)
macro_rules! Depcrate_asciiimpl_24 {
() => {
// Module: crate::ascii
// Provides: {"impl_24"}
// Dependencies: {}
impl < T : AsRef < str > > Ord for Ascii < T > { # [inline] fn cmp (& self , other : & Self) -> Ordering { let self_chars = self . as_ref () . chars () . map (| c | c . to_ascii_lowercase ()) ; let other_chars = other . as_ref () . chars () . map (| c | c . to_ascii_lowercase ()) ; self_chars . cmp (other_chars) } }
};
}
