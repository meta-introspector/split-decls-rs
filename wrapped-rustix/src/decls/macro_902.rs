macro_rules! macro_902 {
    () => {
        bitflags :: bitflags ! { # [doc = " `PIDFD_*` flags for use with [`pidfd_open`]."] # [doc = ""] # [doc = " [`pidfd_open`]: crate::process::pidfd_open"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct PidfdFlags : ffi :: c_uint { # [doc = " `PIDFD_NONBLOCK`"] const NONBLOCK = backend :: c :: PIDFD_NONBLOCK ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_902!()