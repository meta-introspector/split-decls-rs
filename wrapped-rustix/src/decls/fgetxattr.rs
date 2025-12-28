macro_rules! deps {
    () => {
        Result!();
        Arg!();
        Buffer!();
    };
}

macro_rules! fgetxattr {
    () => {
        deps!();
        # [doc = " `fgetxattr(fd, name, value.as_ptr(), value.len())`—Get extended"] # [doc = " filesystem attributes on an open file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fgetxattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fgetxattr.2.html"] # [inline] pub fn fgetxattr < Fd : AsFd , Name : path :: Arg , Buf : Buffer < u8 > > (fd : Fd , name : Name , mut value : Buf ,) -> io :: Result < Buf :: Output > { name . into_with_c_str (| name | { let len = unsafe { backend :: fs :: syscalls :: fgetxattr (fd . as_fd () , name , value . parts_mut ()) ? } ; unsafe { Ok (value . assume_init (len)) } }) }
    };
}

fgetxattr!()