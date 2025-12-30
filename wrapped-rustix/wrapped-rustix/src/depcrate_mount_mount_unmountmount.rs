// Generated macro for mount (function)
macro_rules! Depcrate_mount_mount_unmountmount {
() => {
// Module: crate::mount::mount_unmount
// Provides: {"mount"}
// Dependencies: {}
# [doc = " `mount(source, target, filesystemtype, mountflags, data)`"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mount.2.html"] # [inline] pub fn mount < 'a , Source : path :: Arg , Target : path :: Arg , Fs : path :: Arg , Data : Into < Option < & 'a CStr > > , > (source : Source , target : Target , file_system_type : Fs , flags : MountFlags , data : Data ,) -> io :: Result < () > { source . into_with_c_str (| source | { target . into_with_c_str (| target | { file_system_type . into_with_c_str (| file_system_type | { option_into_with_c_str (data . into () , | data | { backend :: mount :: syscalls :: mount (Some (source) , target , Some (file_system_type) , MountFlagsArg (flags . bits ()) , data ,) }) }) }) }) }
};
}
