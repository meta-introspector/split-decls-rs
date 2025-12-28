macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fsmount {
    () => {
        deps!();
        # [doc = " `fsmount(fs_fd, flags, attr_flags)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsmount.md"] # [inline] pub fn fsmount < Fd : AsFd > (fs_fd : Fd , flags : FsMountFlags , attr_flags : MountAttrFlags ,) -> io :: Result < OwnedFd > { backend :: mount :: syscalls :: fsmount (fs_fd . as_fd () , flags , attr_flags) }
    };
}

fsmount!()