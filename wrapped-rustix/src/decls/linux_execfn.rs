macro_rules! linux_execfn {
    () => {
        # [doc = " `getauxval(AT_EXECFN)`—Returns the Linux \"execfn\" string."] # [doc = ""] # [doc = " Return the string that Linux has recorded as the filesystem path to the"] # [doc = " executable. Returns an empty string if the string is not available."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [cfg (any (linux_raw , any (all (target_os = "android" , target_pointer_width = "64") , target_os = "linux" ,)))] # [inline] pub fn linux_execfn () -> & 'static CStr { backend :: param :: auxv :: linux_execfn () }
    };
}

linux_execfn!();