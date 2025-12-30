// Generated macro for impl_461 (impl)
macro_rules! Depcrate_formatimpl_461 {
() => {
// Module: crate::format
// Provides: {"impl_461"}
// Dependencies: {}
impl < 'a > Write for Buf < 'a > { fn write_str (& mut self , s : & str) -> fmt :: Result { if self . offset + s . len () > self . bytes . len () { Err (fmt :: Error) } else { self . bytes [self . offset .. self . offset + s . len ()] . copy_from_slice (s . as_bytes ()) ; self . offset += s . len () ; Ok (()) } } }
};
}
