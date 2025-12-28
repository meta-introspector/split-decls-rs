macro_rules! deps {
    () => {
        Buffer!();
        Arg!();
        Result!();
    };
}

macro_rules! listxattr {
    () => {
        deps!();
        # [doc = " `listxattr(path, list.as_ptr(), list.len())`—List extended filesystem"] # [doc = " attributes."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/listxattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/listxattr.2.html"] # [inline] pub fn listxattr < P : path :: Arg , Buf : Buffer < u8 > > (path : P , mut list : Buf) -> io :: Result < Buf :: Output > { path . into_with_c_str (| path | { let len = unsafe { backend :: fs :: syscalls :: listxattr (path , list . parts_mut ()) ? } ; unsafe { Ok (list . assume_init (len)) } }) }
    };
}

listxattr!();