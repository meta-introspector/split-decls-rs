// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_io_errorimpl_1068 {
() => {
// Module: crate::io::error
// Provides: {"impl_1068"}
// Dependencies: {}
# [stable (feature = "io_errorkind_display" , since = "1.60.0")] impl fmt :: Display for ErrorKind { # [doc = " Shows a human-readable description of the `ErrorKind`."] # [doc = ""] # [doc = " This is similar to `impl Display for Error`, but doesn't require first converting to Error."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use std::io::ErrorKind;"] # [doc = " assert_eq!(\"entity not found\", ErrorKind::NotFound.to_string());"] # [doc = " ```"] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . write_str (self . as_str ()) } }
};
}
