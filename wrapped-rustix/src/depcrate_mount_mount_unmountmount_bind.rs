// Generated macro for mount_bind (function)
macro_rules! Depcrate_mount_mount_unmountmount_bind {
() => {
// Module: crate::mount::mount_unmount
// Provides: {"mount_bind"}
// Dependencies: {}
# [doc = " `mount(source, target, NULL, MS_BIND, NULL)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html"] # [inline] # [doc (alias = "mount")] # [doc (alias = "MS_BIND")] pub fn mount_bind < Source : path :: Arg , Target : path :: Arg > (source : Source , target : Target ,) -> io :: Result < () > { source . into_with_c_str (| source | { target . into_with_c_str (| target | { backend :: mount :: syscalls :: mount (Some (source) , target , None , MountFlagsArg (MountFlags :: BIND . bits ()) , None ,) }) }) }
};
}
