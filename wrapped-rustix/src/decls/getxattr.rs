macro_rules! deps {
    () => {
        Result!();
        Arg!();
        Buffer!();
    };
}

macro_rules! getxattr {
    () => {
        deps!();
        # [doc = " `getxattr(path, name, value)`—Get extended filesystem attributes."] # [doc = ""] # [doc = " For a higher-level API to xattr functionality, see the [xattr] crate."] # [doc = ""] # [doc = " [xattr]: https://crates.io/crates/xattr"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getxattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getxattr.2.html"] # [inline] pub fn getxattr < P : path :: Arg , Name : path :: Arg , Buf : Buffer < u8 > > (path : P , name : Name , mut value : Buf ,) -> io :: Result < Buf :: Output > { path . into_with_c_str (| path | { name . into_with_c_str (| name | { let len = unsafe { backend :: fs :: syscalls :: getxattr (path , name , value . parts_mut ()) ? } ; unsafe { Ok (value . assume_init (len)) } }) }) }
    };
}

getxattr!()