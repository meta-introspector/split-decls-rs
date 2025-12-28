macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! removexattr {
    () => {
        deps!();
        # [doc = " `removexattr(path, name)`—Remove an extended filesystem attribute."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/removexattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/removexattr.2.html"] pub fn removexattr < P : path :: Arg , Name : path :: Arg > (path : P , name : Name) -> io :: Result < () > { path . into_with_c_str (| path | { name . into_with_c_str (| name | backend :: fs :: syscalls :: removexattr (path , name)) }) }
    };
}

removexattr!();