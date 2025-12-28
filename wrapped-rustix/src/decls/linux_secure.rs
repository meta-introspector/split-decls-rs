macro_rules! linux_secure {
    () => {
        # [doc = " `getauxval(AT_SECURE)`—Returns the Linux “secure execution” mode."] # [doc = ""] # [doc = " Return a boolean value indicating whether “secure execution” mode was"] # [doc = " requested, due to the process having elevated privileges. This includes"] # [doc = " whether the `AT_SECURE` AUX value is set, and whether the initial real UID"] # [doc = " and GID differ from the initial effective UID and GID."] # [doc = ""] # [doc = " The meaning of “secure execution” mode is beyond the scope of this"] # [doc = " comment."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [inline] pub fn linux_secure () -> bool { backend :: param :: auxv :: linux_secure () }
    };
}

linux_secure!()