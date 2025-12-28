macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unshare {
    () => {
        deps!();
        # [doc = " `unshare(flags)`—Deprecated in favor of [`unshare_unsafe`]."] # [doc = ""] # [doc = " This function should be unsafe; see the safety comment on `unshare_unsafe`."] # [deprecated (since = "1.1.0" , note = "Use `unshare_unsafe`")] pub fn unshare (flags : UnshareFlags) -> io :: Result < () > { unsafe { syscalls :: unshare (flags) } }
    };
}

unshare!();