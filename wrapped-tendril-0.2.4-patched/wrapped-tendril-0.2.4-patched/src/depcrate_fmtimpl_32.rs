// Generated macro for impl_32 (impl)
macro_rules! Depcrate_fmtimpl_32 {
() => {
// Module: crate::fmt
// Provides: {"impl_32"}
// Dependencies: {}
unsafe impl Format for UTF8 { # [inline] fn validate (buf : & [u8]) -> bool { str :: from_utf8 (buf) . is_ok () } # [inline] fn validate_prefix (buf : & [u8]) -> bool { if buf . len () == 0 { return true ; } match futf :: classify (buf , buf . len () - 1) { Some (Codepoint { meaning : Meaning :: Whole (_) , .. }) => true , _ => false , } } # [inline] fn validate_suffix (buf : & [u8]) -> bool { if buf . len () == 0 { return true ; } match futf :: classify (buf , 0) { Some (Codepoint { meaning : Meaning :: Whole (_) , .. }) => true , _ => false , } } # [inline] fn validate_subseq (buf : & [u8]) -> bool { < Self as Format > :: validate_prefix (buf) && < Self as Format > :: validate_suffix (buf) } }
};
}
