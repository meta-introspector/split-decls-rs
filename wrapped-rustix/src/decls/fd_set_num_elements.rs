macro_rules! fd_set_num_elements {
    () => {
        # [doc = " Compute the number of `FdSetElement`s needed to hold a set which can"] # [doc = " contain up to `set_count` file descriptors with values less than `nfds`."] # [inline] pub fn fd_set_num_elements (set_count : usize , nfds : RawFd) -> usize { # [cfg (any (windows , target_os = "wasi"))] { let _ = nfds ; fd_set_num_elements_for_fd_array (set_count) } # [cfg (not (any (windows , target_os = "wasi")))] { let _ = set_count ; fd_set_num_elements_for_bitvector (nfds) } }
    };
}

fd_set_num_elements!()