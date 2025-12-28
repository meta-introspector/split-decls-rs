macro_rules! deps {
    () => {
        FdSetElement!();
        FD_SET!();
    };
}

macro_rules! fd_set_bound {
    () => {
        deps!();
        # [doc = " Compute the minimum `nfds` value needed for the set pointed to by `fds`."] # [inline] pub fn fd_set_bound (fds : & [FdSetElement]) -> RawFd { # [cfg (not (any (windows , target_os = "wasi")))] { if let Some (position) = fds . iter () . rposition (| element | element . 0 != 0) { let element = fds [position] . 0 ; (position * BITS + (BITS - element . leading_zeros () as usize)) as RawFd } else { 0 } } # [cfg (any (windows , target_os = "wasi"))] { let set = unsafe { & * fds . as_ptr () . cast :: < FD_SET > () } ; let fd_count = set . fd_count ; let fd_array = & set . fd_array [.. fd_count as usize] ; let mut max = 0 ; for fd in fd_array { if * fd >= max { max = * fd + 1 ; } } max as RawFd } }
    };
}

fd_set_bound!();