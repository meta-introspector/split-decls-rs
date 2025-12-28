macro_rules! DEFAULT_SECRET {
    () => {
        pub const DEFAULT_SECRET : & Secret = unsafe { Secret :: new_unchecked (& DEFAULT_SECRET_RAW) } ;
    };
}

DEFAULT_SECRET!();