macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! unmount {
    () => {
        deps!();
        # [doc = " `umount2(target, flags)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/umount.2.html"] # [inline] # [doc (alias = "umount" , alias = "umount2")] pub fn unmount < Target : path :: Arg > (target : Target , flags : UnmountFlags) -> io :: Result < () > { target . into_with_c_str (| target | backend :: mount :: syscalls :: unmount (target , flags)) }
    };
}

unmount!();