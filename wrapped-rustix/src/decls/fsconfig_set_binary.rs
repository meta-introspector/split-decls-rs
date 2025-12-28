macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! fsconfig_set_binary {
    () => {
        deps!();
        # [doc = " `fsconfig(fs_fd, FSCONFIG_SET_BINARY, key, value, value.len())`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_set_binary < Key : path :: Arg , Fd : AsFd > (fs_fd : Fd , key : Key , value : & [u8] ,) -> io :: Result < () > { let fs_fd = fs_fd . as_fd () ; key . into_with_c_str (| key | backend :: mount :: syscalls :: fsconfig_set_binary (fs_fd , key , value)) }
    };
}

fsconfig_set_binary!()