macro_rules! deps {
    () => {
        FdSetElement!();
    };
}

macro_rules! FdSetIter {
    () => {
        deps!();
        # [doc = " An iterator over the fds in a set."] # [doc (alias = "FD_ISSET")] # [cfg (any (windows , target_os = "wasi"))] pub struct FdSetIter < 'a > { current : usize , fds : & 'a [FdSetElement] , }
    };
}

FdSetIter!();