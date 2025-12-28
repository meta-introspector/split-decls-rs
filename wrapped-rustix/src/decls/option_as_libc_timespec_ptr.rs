macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! option_as_libc_timespec_ptr {
    () => {
        deps!();
        # [cfg (not (fix_y2038))] pub (crate) fn option_as_libc_timespec_ptr (timespec : Option < & Timespec >) -> * const c :: timespec { match timespec { None => null () , Some (timespec) => as_libc_timespec_ptr (timespec) , } }
    };
}

option_as_libc_timespec_ptr!();