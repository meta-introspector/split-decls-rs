macro_rules! deps {
    () => {
        Result!();
        Buffer!();
        Arg!();
    };
}

macro_rules! lgetxattr {
    () => {
        deps!();
        # [doc = " `lgetxattr(path, name, value.as_ptr(), value.len())`—Get extended"] # [doc = " filesystem attributes, without following symlinks in the last path"] # [doc = " component."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/lgetxattr.2.html"] # [inline] pub fn lgetxattr < P : path :: Arg , Name : path :: Arg , Buf : Buffer < u8 > > (path : P , name : Name , mut value : Buf ,) -> io :: Result < Buf :: Output > { path . into_with_c_str (| path | { name . into_with_c_str (| name | { let len = unsafe { backend :: fs :: syscalls :: lgetxattr (path , name , value . parts_mut ()) ? } ; unsafe { Ok (value . assume_init (len)) } }) }) }
    };
}

lgetxattr!();