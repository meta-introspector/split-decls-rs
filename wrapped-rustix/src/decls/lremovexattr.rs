macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! lremovexattr {
    () => {
        deps!();
        # [doc = " `lremovexattr(path, name)`—Remove an extended filesystem attribute,"] # [doc = " without following symlinks in the last path component."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/lremovexattr.2.html"] pub fn lremovexattr < P : path :: Arg , Name : path :: Arg > (path : P , name : Name) -> io :: Result < () > { path . into_with_c_str (| path | { name . into_with_c_str (| name | backend :: fs :: syscalls :: lremovexattr (path , name)) }) }
    };
}

lremovexattr!()