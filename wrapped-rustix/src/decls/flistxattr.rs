macro_rules! deps {
    () => {
        Buffer!();
        Result!();
    };
}

macro_rules! flistxattr {
    () => {
        deps!();
        # [doc = " `flistxattr(fd, list.as_ptr(), list.len())`—List extended filesystem"] # [doc = " attributes on an open file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/flistxattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flistxattr.2.html"] # [inline] pub fn flistxattr < Fd : AsFd , Buf : Buffer < u8 > > (fd : Fd , mut list : Buf) -> io :: Result < Buf :: Output > { let len = unsafe { backend :: fs :: syscalls :: flistxattr (fd . as_fd () , list . parts_mut ()) ? } ; unsafe { Ok (list . assume_init (len)) } }
    };
}

flistxattr!();