// Generated macro for impl_1069 (impl)
macro_rules! Depcrate_io_errorimpl_1069 {
() => {
// Module: crate::io::error
// Provides: {"impl_1069"}
// Dependencies: {}
# [doc = " Intended for use for errors not exposed to the user, where allocating onto"] # [doc = " the heap (for normal construction via Error::new) is too costly."] # [stable (feature = "io_error_from_errorkind" , since = "1.14.0")] impl From < ErrorKind > for Error { # [doc = " Converts an [`ErrorKind`] into an [`Error`]."] # [doc = ""] # [doc = " This conversion creates a new error with a simple representation of error kind."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::{Error, ErrorKind};"] # [doc = ""] # [doc = " let not_found = ErrorKind::NotFound;"] # [doc = " let error = Error::from(not_found);"] # [doc = " assert_eq!(\"entity not found\", format!(\"{error}\"));"] # [doc = " ```"] # [inline] fn from (kind : ErrorKind) -> Error { Error { repr : Repr :: new_simple (kind) } } }
};
}
