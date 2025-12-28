macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! fsetxattr {
    () => {
        deps!();
        # [doc = " `fsetxattr(fd, name, value.as_ptr(), value.len(), flags)`—Set extended"] # [doc = " filesystem attributes on an open file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fsetxattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsetxattr.2.html"] # [inline] pub fn fsetxattr < Fd : AsFd , Name : path :: Arg > (fd : Fd , name : Name , value : & [u8] , flags : XattrFlags ,) -> io :: Result < () > { name . into_with_c_str (| name | backend :: fs :: syscalls :: fsetxattr (fd . as_fd () , name , value , flags)) }
    };
}

fsetxattr!();