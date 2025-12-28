macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! as_libc_timespec_mut_ptr {
    () => {
        deps!();
        # [cfg (not (fix_y2038))] pub (crate) fn as_libc_timespec_mut_ptr (timespec : & mut core :: mem :: MaybeUninit < Timespec > ,) -> * mut c :: timespec { # [cfg (test)] { assert_eq_size ! (Timespec , c :: timespec) ; } timespec . as_mut_ptr () . cast :: < c :: timespec > () }
    };
}

as_libc_timespec_mut_ptr!()