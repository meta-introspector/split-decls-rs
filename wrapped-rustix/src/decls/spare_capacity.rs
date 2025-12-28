macro_rules! deps {
    () => {
        SpareCapacity!();
        Buffer!();
    };
}

macro_rules! spare_capacity {
    () => {
        deps!();
        # [doc = " Construct an [`SpareCapacity`], which implements [`Buffer`]."] # [doc = ""] # [doc = " This wraps a `Vec` and uses the spare capacity of the `Vec` as the buffer"] # [doc = " to receive data in, automatically calling `set_len` on the `Vec` to set the"] # [doc = " length to include the received elements."] # [doc = ""] # [doc = " This uses the existing capacity, and never allocates, so the `Vec` should"] # [doc = " have some non-empty spare capacity!"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn test(input: rustix::fd::BorrowedFd) -> rustix::io::Result<()> {"] # [doc = " use rustix::buffer::spare_capacity;"] # [doc = " use rustix::io::{read, Errno};"] # [doc = ""] # [doc = " let mut buf = Vec::with_capacity(1024);"] # [doc = " match read(input, spare_capacity(&mut buf)) {"] # [doc = "     Ok(0) => { /* end of stream */ }"] # [doc = "     Ok(n) => { /* `buf` is now `n` bytes longer */ }"] # [doc = "     Err(Errno::INTR) => { /* `buf` is unmodified */ }"] # [doc = "     Err(e) => {"] # [doc = "         return Err(e);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn spare_capacity < 'a , T > (v : & 'a mut Vec < T >) -> SpareCapacity < 'a , T > { debug_assert_ne ! (v . capacity () , 0 , "`extend` uses spare capacity, and never allocates new memory, so the `Vec` passed to it \
         should have some spare capacity.") ; SpareCapacity (v) }
    };
}

spare_capacity!();