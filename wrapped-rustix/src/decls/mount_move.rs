macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! mount_move {
    () => {
        deps!();
        # [doc = " `mount(source, target, NULL, MS_MOVE, NULL)`"] # [doc = ""] # [doc = " This is not the same as the `move_mount` syscall. If you want to use that,"] # [doc = " use [`move_mount`] instead."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [`move_mount`]: crate::mount::move_mount"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html"] # [inline] # [doc (alias = "mount")] # [doc (alias = "MS_MOVE")] pub fn mount_move < Source : path :: Arg , Target : path :: Arg > (source : Source , target : Target ,) -> io :: Result < () > { source . into_with_c_str (| source | { target . into_with_c_str (| target | { backend :: mount :: syscalls :: mount (Some (source) , target , None , MountFlagsArg (InternalMountFlags :: MOVE . bits ()) , None ,) }) }) }
    };
}

mount_move!();