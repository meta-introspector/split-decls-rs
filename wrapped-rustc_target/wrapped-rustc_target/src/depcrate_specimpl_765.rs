// Generated macro for impl_765 (impl)
macro_rules! Depcrate_specimpl_765 {
() => {
// Module: crate::spec
// Provides: {"impl_765"}
// Dependencies: {}
# [doc = " Formats a sanitizer set as a comma separated list of sanitizers' names."] impl fmt :: Display for SanitizerSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut first = true ; for s in * self { let name = s . as_str () . unwrap_or_else (| | panic ! ("unrecognized sanitizer {s:?}")) ; if ! first { f . write_str (", ") ? ; } f . write_str (name) ? ; first = false ; } Ok (()) } }
};
}
