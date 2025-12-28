macro_rules! deps {
    () => {
        Arg!();
        Result!();
    };
}

macro_rules! mount_remount {
    () => {
        deps!();
        # [doc = " `mount(NULL, target, NULL, MS_REMOUNT | mountflags, data)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html"] # [inline] # [doc (alias = "mount")] # [doc (alias = "MS_REMOUNT")] pub fn mount_remount < Target : path :: Arg , Data : path :: Arg > (target : Target , flags : MountFlags , data : Data ,) -> io :: Result < () > { target . into_with_c_str (| target | { data . into_with_c_str (| data | { backend :: mount :: syscalls :: mount (None , target , None , MountFlagsArg (InternalMountFlags :: REMOUNT . bits () | flags . bits ()) , Some (data) ,) }) }) }
    };
}

mount_remount!()