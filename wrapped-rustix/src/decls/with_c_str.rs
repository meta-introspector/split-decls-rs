macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! with_c_str {
    () => {
        deps!();
        # [doc = " Runs a closure with `bytes` passed in as a `&CStr`."] # [allow (unsafe_code , clippy :: int_plus_one)] # [inline] fn with_c_str < T , F > (bytes : & [u8] , f : F) -> io :: Result < T > where F : FnOnce (& CStr) -> io :: Result < T > , { if bytes . len () >= SMALL_PATH_BUFFER_SIZE { return with_c_str_slow_path (bytes , f) ; } let mut buf = MaybeUninit :: < [u8 ; SMALL_PATH_BUFFER_SIZE] > :: uninit () ; let buf_ptr = buf . as_mut_ptr () . cast :: < u8 > () ; debug_assert ! (bytes . len () + 1 <= SMALL_PATH_BUFFER_SIZE) ; unsafe { ptr :: copy_nonoverlapping (bytes . as_ptr () , buf_ptr , bytes . len ()) ; buf_ptr . add (bytes . len ()) . write (b'\0') ; } match CStr :: from_bytes_with_nul (unsafe { slice :: from_raw_parts (buf_ptr , bytes . len () + 1) }) { Ok (s) => f (s) , Err (_) => Err (io :: Errno :: INVAL) , } }
    };
}

with_c_str!();