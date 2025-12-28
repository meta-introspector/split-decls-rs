macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! as_libc_timespec_ptr {
    () => {
        deps!();
        # [cfg (not (fix_y2038))] pub (crate) fn as_libc_timespec_ptr (timespec : & Timespec) -> * const c :: timespec { # [cfg (test)] { assert_eq_size ! (Timespec , c :: timespec) ; } crate :: utils :: as_ptr (timespec) . cast :: < c :: timespec > () }
    };
}

as_libc_timespec_ptr!();