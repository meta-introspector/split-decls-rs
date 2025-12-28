macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! mount_change {
    () => {
        deps!();
        # [doc = " `mount(NULL, target, NULL, mountflags, NULL)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html"] # [inline] # [doc (alias = "mount")] pub fn mount_change < Target : path :: Arg > (target : Target , flags : MountPropagationFlags ,) -> io :: Result < () > { target . into_with_c_str (| target | { backend :: mount :: syscalls :: mount (None , target , None , MountFlagsArg (flags . bits ()) , None) }) }
    };
}

mount_change!()