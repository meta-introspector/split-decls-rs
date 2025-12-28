macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! fsconfig_set_string {
    () => {
        deps!();
        # [doc = " `fsconfig(fs_fd, FSCONFIG_SET_STRING, key, value, 0)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_set_string < Key : path :: Arg , Value : path :: Arg , Fd : AsFd > (fs_fd : Fd , key : Key , value : Value ,) -> io :: Result < () > { let fs_fd = fs_fd . as_fd () ; key . into_with_c_str (| key | { value . into_with_c_str (| value | { backend :: mount :: syscalls :: fsconfig_set_string (fs_fd , key , value) }) }) }
    };
}

fsconfig_set_string!()