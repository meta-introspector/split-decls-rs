macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! fremovexattr {
    () => {
        deps!();
        # [doc = " `fremovexattr(fd, name)`—Remove an extended filesystem attribute on an"] # [doc = " open file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fremovexattr.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fremovexattr.2.html"] pub fn fremovexattr < Fd : AsFd , Name : path :: Arg > (fd : Fd , name : Name) -> io :: Result < () > { name . into_with_c_str (| name | backend :: fs :: syscalls :: fremovexattr (fd . as_fd () , name)) }
    };
}

fremovexattr!();