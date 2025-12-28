macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! fsconfig_set_fd {
    () => {
        deps!();
        # [doc = " `fsconfig(fs_fd, FSCONFIG_SET_FD, key, NULL, fd)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_set_fd < Key : path :: Arg , Fd : AsFd , AuxFd : AsFd > (fs_fd : Fd , key : Key , fd : AuxFd ,) -> io :: Result < () > { let fs_fd = fs_fd . as_fd () ; let fd = fd . as_fd () ; key . into_with_c_str (| key | backend :: mount :: syscalls :: fsconfig_set_fd (fs_fd , key , fd)) }
    };
}

fsconfig_set_fd!();