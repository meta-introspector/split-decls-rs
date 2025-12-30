// Generated macro for _readlink (function)
macro_rules! Depcrate_fs_abs_readlink {
() => {
// Module: crate::fs::abs
// Provides: {"_readlink"}
// Dependencies: {}
# [cfg (feature = "alloc")] fn _readlink (path : & CStr , mut buffer : Vec < u8 >) -> io :: Result < CString > { buffer . clear () ; buffer . reserve (SMALL_PATH_BUFFER_SIZE) ; buffer . resize (buffer . capacity () , 0_u8) ; loop { let nread = backend :: fs :: syscalls :: readlink (path , & mut buffer) ? ; let nread = nread as usize ; assert ! (nread <= buffer . len ()) ; if nread < buffer . len () { buffer . resize (nread , 0_u8) ; return Ok (CString :: new (buffer) . unwrap ()) ; } buffer . reserve (1) ; buffer . resize (buffer . capacity () , 0_u8) ; } }
};
}
