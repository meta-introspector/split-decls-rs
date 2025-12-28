macro_rules! deps {
    () => {
        FD_SET!();
        FdSetElement!();
    };
}

macro_rules! fd_set_num_elements_for_fd_array_raw {
    () => {
        deps!();
        # [doc = " Compute the raw `fd_set_num_elements` value, before ensuring the value is"] # [doc = " big enough to dereference an `FD_SET`."] # [cfg (any (windows , target_os = "wasi"))] # [inline] fn fd_set_num_elements_for_fd_array_raw (set_count : usize) -> usize { div_ceil (core :: cmp :: max (align_of :: < FD_SET > () , align_of :: < RawFd > ()) + set_count * size_of :: < RawFd > () , size_of :: < FdSetElement > () ,) }
    };
}

fd_set_num_elements_for_fd_array_raw!();