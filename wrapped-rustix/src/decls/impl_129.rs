macro_rules! deps {
    () => {
        FD_SET!();
        FdSetIter!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        # [cfg (any (windows , target_os = "wasi"))] impl < 'a > Iterator for FdSetIter < 'a > { type Item = RawFd ; fn next (& mut self) -> Option < Self :: Item > { let current = self . current ; let set = unsafe { & * self . fds . as_ptr () . cast :: < FD_SET > () } ; let fd_count = set . fd_count ; let fd_array = & set . fd_array [.. fd_count as usize] ; if current == fd_count as usize { return None ; } let fd = fd_array [current as usize] ; self . current = current + 1 ; Some (fd as RawFd) } }
    };
}

impl_129!()