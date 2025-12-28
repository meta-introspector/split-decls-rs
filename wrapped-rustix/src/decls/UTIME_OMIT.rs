macro_rules! deps {
    () => {
        Nsecs!();
    };
}

macro_rules! UTIME_OMIT {
    () => {
        deps!();
        # [doc = " `UTIME_OMIT` for use with [`utimensat`]."] # [doc = ""] # [doc = " [`utimensat`]: crate::fs::utimensat"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "redox" , target_os = "vita")))] pub const UTIME_OMIT : Nsecs = backend :: c :: UTIME_OMIT as Nsecs ;
    };
}

UTIME_OMIT!()