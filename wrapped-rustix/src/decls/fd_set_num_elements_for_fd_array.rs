macro_rules! deps {
    () => {
        FD_SET!();
        FdSetElement!();
    };
}

macro_rules! fd_set_num_elements_for_fd_array {
    () => {
        deps!();
        # [doc = " `fd_set_num_elements` implementation on platforms with fd array"] # [doc = " implementations."] # [cfg (any (windows , target_os = "wasi"))] # [inline] pub (crate) fn fd_set_num_elements_for_fd_array (set_count : usize) -> usize { core :: cmp :: max (fd_set_num_elements_for_fd_array_raw (set_count) , div_ceil (size_of :: < FD_SET > () , size_of :: < FdSetElement > ()) ,) }
    };
}

fd_set_num_elements_for_fd_array!()