macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! Nsecs {
    () => {
        deps!();
        # [doc = " A type for the `tv_nsec` field of [`Timespec`]."] # [cfg (all (not (fix_y2038) , libc , not (all (target_arch = "x86_64" , target_pointer_width = "32"))))] pub type Nsecs = ffi :: c_long ;
    };
}

Nsecs!()