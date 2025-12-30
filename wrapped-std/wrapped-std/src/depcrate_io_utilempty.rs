// Generated macro for empty (function)
macro_rules! Depcrate_io_utilempty {
() => {
// Module: crate::io::util
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Creates a value that is always at EOF for reads, and ignores all data written."] # [doc = ""] # [doc = " All calls to [`write`] on the returned instance will return [`Ok(buf.len())`]"] # [doc = " and the contents of the buffer will not be inspected."] # [doc = ""] # [doc = " All calls to [`read`] from the returned reader will return [`Ok(0)`]."] # [doc = ""] # [doc = " [`Ok(buf.len())`]: Ok"] # [doc = " [`Ok(0)`]: Ok"] # [doc = ""] # [doc = " [`write`]: Write::write"] # [doc = " [`read`]: Read::read"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::io::{self, Write};"] # [doc = ""] # [doc = " let buffer = vec![1, 2, 3, 5, 8];"] # [doc = " let num_bytes = io::empty().write(&buffer).unwrap();"] # [doc = " assert_eq!(num_bytes, 5);"] # [doc = " ```"] # [doc = ""] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::io::{self, Read};"] # [doc = ""] # [doc = " let mut buffer = String::new();"] # [doc = " io::empty().read_to_string(&mut buffer).unwrap();"] # [doc = " assert!(buffer.is_empty());"] # [doc = " ```"] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_stable (feature = "const_io_structs" , since = "1.79.0")] pub const fn empty () -> Empty { Empty }
};
}
