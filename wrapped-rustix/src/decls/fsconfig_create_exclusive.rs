macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fsconfig_create_exclusive {
    () => {
        deps!();
        # [doc = " `fsconfig(fs_fd, FSCONFIG_CMD_CREATE_EXCL, key, NULL, 0)`"] # [doc = ""] # [doc = " This function was added in Linux 6.6."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/fsconfig.md"] # [inline] # [doc (alias = "fsconfig")] pub fn fsconfig_create_exclusive < Fd : AsFd > (fs_fd : Fd) -> io :: Result < () > { backend :: mount :: syscalls :: fsconfig_create_excl (fs_fd . as_fd ()) }
    };
}

fsconfig_create_exclusive!()