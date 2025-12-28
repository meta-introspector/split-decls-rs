macro_rules! linux_hwcap {
    () => {
        # [doc = " `(getauxval(AT_HWCAP), getauxval(AT_HWCAP2)`—Returns the Linux \"hwcap\""] # [doc = " data."] # [doc = ""] # [doc = " Return the Linux `AT_HWCAP` and `AT_HWCAP2` values passed to the"] # [doc = " current process. Returns 0 for each value if it is not available."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [cfg (any (linux_raw , any (all (target_os = "android" , target_pointer_width = "64") , target_os = "linux" ,)))] # [inline] pub fn linux_hwcap () -> (usize , usize) { backend :: param :: auxv :: linux_hwcap () }
    };
}

linux_hwcap!();