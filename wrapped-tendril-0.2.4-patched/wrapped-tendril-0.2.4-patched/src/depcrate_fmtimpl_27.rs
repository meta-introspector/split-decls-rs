// Generated macro for impl_27 (impl)
macro_rules! Depcrate_fmtimpl_27 {
() => {
// Module: crate::fmt
// Provides: {"impl_27"}
// Dependencies: {}
unsafe impl Format for ASCII { # [inline] fn validate (buf : & [u8]) -> bool { buf . iter () . all (| & n | n <= 127) } # [inline (always)] fn validate_prefix (_ : & [u8]) -> bool { true } # [inline (always)] fn validate_suffix (_ : & [u8]) -> bool { true } # [inline (always)] fn validate_subseq (_ : & [u8]) -> bool { true } }
};
}
