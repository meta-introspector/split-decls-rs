macro_rules! deps {
    () => {
        FD_SET!();
        FdSetElement!();
    };
}

macro_rules! fd_set_remove {
    () => {
        deps!();
        # [doc = " Clear `fd` in the set pointed to by `fds`."] # [doc (alias = "FD_CLR")] # [inline] pub fn fd_set_remove (fds : & mut [FdSetElement] , fd : RawFd) { # [cfg (not (any (windows , target_os = "wasi")))] { let fd = fd as usize ; fds [fd / BITS] . 0 &= ! (1 << (fd % BITS)) ; } # [cfg (any (windows , target_os = "wasi"))] { let set = unsafe { & mut * fds . as_mut_ptr () . cast :: < FD_SET > () } ; let fd_count = set . fd_count ; let fd_array = & set . fd_array [.. fd_count as usize] ; if let Some (pos) = fd_array . iter () . position (| p | * p as RawFd == fd) { set . fd_count = fd_count - 1 ; set . fd_array [pos] = * set . fd_array . last () . unwrap () ; } } }
    };
}

fd_set_remove!();