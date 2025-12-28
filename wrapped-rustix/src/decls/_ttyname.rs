macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! _ttyname {
    () => {
        deps!();
        # [cfg (not (any (target_os = "fuchsia" , target_os = "wasi")))] # [cfg (feature = "alloc")] # [cfg (feature = "fs")] # [allow (unsafe_code)] fn _ttyname (fd : BorrowedFd < '_ > , mut buffer : Vec < u8 >) -> io :: Result < CString > { buffer . clear () ; buffer . reserve (SMALL_PATH_BUFFER_SIZE) ; loop { match backend :: termios :: syscalls :: ttyname (fd , buffer . spare_capacity_mut ()) { Err (io :: Errno :: RANGE) => { buffer . reserve (buffer . capacity () + 1) ; } Ok (len) => { unsafe { buffer . set_len (len + 1) ; } unsafe { return Ok (CString :: from_vec_with_nul_unchecked (buffer)) ; } } Err (errno) => return Err (errno) , } } }
    };
}

_ttyname!()