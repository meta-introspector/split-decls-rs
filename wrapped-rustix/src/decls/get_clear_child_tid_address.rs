macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! get_clear_child_tid_address {
    () => {
        deps!();
        # [doc = " Get the `clear_child_tid` address set by `set_tid_address`"] # [doc = " and `clone`'s `CLONE_CHILD_CLEARTID` flag."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TID_ADDRESS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TID_ADDRESS,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] pub fn get_clear_child_tid_address () -> io :: Result < Option < NonNull < c_void > > > { unsafe { prctl_get_at_arg2_optional :: < * mut c_void > (PR_GET_TID_ADDRESS) } . map (NonNull :: new) }
    };
}

get_clear_child_tid_address!()