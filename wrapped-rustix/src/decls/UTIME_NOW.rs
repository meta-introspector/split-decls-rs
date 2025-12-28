macro_rules! deps {
    () => {
        Nsecs!();
    };
}

macro_rules! UTIME_NOW {
    () => {
        deps!();
        # [doc = " `UTIME_NOW` for use with [`utimensat`]."] # [doc = ""] # [doc = " [`utimensat`]: crate::fs::utimensat"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "redox" , target_os = "vita")))] pub const UTIME_NOW : Nsecs = backend :: c :: UTIME_NOW as Nsecs ;
    };
}

UTIME_NOW!()