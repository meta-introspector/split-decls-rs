// Generated macro for _readlinkat (function)
macro_rules! Depcrate_fs_at_readlinkat {
() => {
// Module: crate::fs::at
// Provides: {"_readlinkat"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [allow (unsafe_code)] fn _readlinkat (dirfd : BorrowedFd < '_ > , path : & CStr , mut buffer : Vec < u8 >) -> io :: Result < CString > { buffer . clear () ; buffer . reserve (SMALL_PATH_BUFFER_SIZE) ; loop { let buf = buffer . spare_capacity_mut () ; let nread = unsafe { backend :: fs :: syscalls :: readlinkat (dirfd . as_fd () , path , (buf . as_mut_ptr () . cast () , buf . len ()) ,) ? } ; debug_assert ! (nread <= buffer . capacity ()) ; if nread < buffer . capacity () { unsafe { buffer . set_len (nread) ; } unsafe { return Ok (CString :: from_vec_unchecked (buffer)) ; } } buffer . reserve (buffer . capacity () + 1) ; } }
};
}
