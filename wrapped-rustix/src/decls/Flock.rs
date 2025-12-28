macro_rules! deps {
    () => {
        FlockType!();
        FlockOffsetType!();
        Pid!();
    };
}

macro_rules! Flock {
    () => {
        deps!();
        # [doc = " File lock data structure used in [`fcntl_getlk`]."] # [doc = ""] # [doc = " [`fcntl_getlk`]: crate::process::fcntl_getlk()"] # [cfg (not (target_os = "horizon"))] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct Flock { # [doc = " Starting offset for lock"] pub start : u64 , # [doc = " Number of bytes to lock"] pub length : u64 , # [doc = " PID of process blocking our lock. If set to `None`, it refers to the"] # [doc = " current process"] pub pid : Option < Pid > , # [doc = " Type of lock"] pub typ : FlockType , # [doc = " Offset type of lock"] pub offset_type : FlockOffsetType , }
    };
}

Flock!()