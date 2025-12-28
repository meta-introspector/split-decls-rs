macro_rules! deps {
    () => {
        Statx!();
    };
}

macro_rules! StatxTimestamp {
    () => {
        deps!();
        # [doc = " `struct statx_timestamp` for use with [`Statx`]."] # [repr (C)] # [derive (Debug , Copy , Clone)] # [non_exhaustive] pub struct StatxTimestamp { # [doc = " Seconds."] pub tv_sec : i64 , # [doc = " Nanoseconds. Must be less than 1_000_000_000."] pub tv_nsec : u32 , pub (crate) __reserved : i32 , }
    };
}

StatxTimestamp!();