macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! move_mount {
    () => {
        deps!();
        # [doc = " `move_mount(from_dfd, from_pathname, to_dfd, to_pathname, flags)`"] # [doc = ""] # [doc = " This is not the same as `mount` with the `MS_MOVE` flag. If you want to"] # [doc = " use that, use [`mount_move`] instead."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Unfinished draft]"] # [doc = ""] # [doc = " [`mount_move`]: crate::mount::mount_move"] # [doc = " [Unfinished draft]: https://github.com/sunfishcode/linux-mount-api-documentation/blob/main/move_mount.md"] # [inline] pub fn move_mount < From : path :: Arg , To : path :: Arg , FromFd : AsFd , ToFd : AsFd > (from_dfd : FromFd , from_pathname : From , to_dfd : ToFd , to_pathname : To , flags : MoveMountFlags ,) -> io :: Result < () > { let from_dfd = from_dfd . as_fd () ; let to_dfd = to_dfd . as_fd () ; from_pathname . into_with_c_str (| from_pathname | { to_pathname . into_with_c_str (| to_pathname | { backend :: mount :: syscalls :: move_mount (from_dfd , from_pathname , to_dfd , to_pathname , flags ,) }) }) }
    };
}

move_mount!()