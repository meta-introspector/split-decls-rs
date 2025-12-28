macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! setxattr {
    () => {
        deps!();
        # [doc = " `setxattr(path, name, value.as_ptr(), value.len(), flags)`—Set extended"] # [doc = " filesystem attributes."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setxattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setxattr.2.html"] # [inline] pub fn setxattr < P : path :: Arg , Name : path :: Arg > (path : P , name : Name , value : & [u8] , flags : XattrFlags ,) -> io :: Result < () > { path . into_with_c_str (| path | { name . into_with_c_str (| name | backend :: fs :: syscalls :: setxattr (path , name , value , flags)) }) }
    };
}

setxattr!();