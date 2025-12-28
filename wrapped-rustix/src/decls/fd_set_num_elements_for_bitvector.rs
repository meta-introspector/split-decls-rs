macro_rules! fd_set_num_elements_for_bitvector {
    () => {
        # [doc = " `fd_set_num_elements` implementation on platforms with bitvector"] # [doc = " implementations."] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] pub (crate) fn fd_set_num_elements_for_bitvector (nfds : RawFd) -> usize { let nfds = nfds as usize ; div_ceil (nfds , BITS) }
    };
}

fd_set_num_elements_for_bitvector!();