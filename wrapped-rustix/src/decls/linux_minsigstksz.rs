macro_rules! linux_minsigstksz {
    () => {
        # [doc = " `getauxval(AT_MINSIGSTKSZ)`—Returns the Linux \"minsigstksz\" data."] # [doc = ""] # [doc = " Return the Linux `AT_MINSIGSTKSZ` value passed to the current process."] # [doc = " Returns 0 if it is not available."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [cfg (any (linux_raw , any (all (target_os = "android" , target_pointer_width = "64") , target_os = "linux" ,)))] # [inline] pub fn linux_minsigstksz () -> usize { backend :: param :: auxv :: linux_minsigstksz () }
    };
}

linux_minsigstksz!()