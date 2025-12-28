macro_rules! deps {
    () => {
        Nsecs!();
        Secs!();
    };
}

macro_rules! Timespec {
    () => {
        deps!();
        # [doc = " `struct timespec`—A quantity of time in seconds plus nanoseconds."] # [derive (Debug , Clone , Copy , Default , PartialEq , Eq , PartialOrd , Ord)] # [repr (C)] pub struct Timespec { # [doc = " Seconds."] pub tv_sec : Secs , # [doc = " Nanoseconds. Must be less than 1_000_000_000."] # [doc = ""] # [doc = " When passed to [`rustix::fs::utimensat`], this field may instead be"] # [doc = " assigned the values [`UTIME_NOW`] or [`UTIME_OMIT`]."] # [doc = ""] # [doc = " [`UTIME_NOW`]: crate::fs::UTIME_NOW"] # [doc = " [`UTIME_OMIT`]: crate::fs::UTIME_OMIT"] # [doc = " [`rustix::fs::utimensat`]: crate::fs::utimensat"] pub tv_nsec : Nsecs , }
    };
}

Timespec!();