macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! fsconfig_set_path_empty {
    () => {
        deps!();
        # [doc = " `fsconfig(fs_fd, FSCONFIG_SET_PATH_EMPTY, key, \"\", fd)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_set_path_empty < Key : path :: Arg , Fd : AsFd , AuxFd : AsFd > (fs_fd : Fd , key : Key , fd : AuxFd ,) -> io :: Result < () > { let fs_fd = fs_fd . as_fd () ; let fd = fd . as_fd () ; key . into_with_c_str (| key | backend :: mount :: syscalls :: fsconfig_set_path_empty (fs_fd , key , fd)) }
    };
}

fsconfig_set_path_empty!()