macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! _getcwd {
    () => {
        deps!();
        # [cfg (all (feature = "alloc" , feature = "fs"))] # [allow (unsafe_code)] fn _getcwd (mut buffer : Vec < u8 >) -> io :: Result < CString > { buffer . clear () ; buffer . reserve (SMALL_PATH_BUFFER_SIZE) ; loop { match backend :: process :: syscalls :: getcwd (buffer . spare_capacity_mut ()) { Err (io :: Errno :: RANGE) => { buffer . reserve (buffer . capacity () + 1) ; } Ok (_) => { unsafe { buffer . set_len (CStr :: from_ptr (buffer . as_ptr () . cast ()) . to_bytes_with_nul () . len () ,) ; return Ok (CString :: from_vec_with_nul_unchecked (buffer)) ; } } Err (errno) => return Err (errno) , } } }
    };
}

_getcwd!();