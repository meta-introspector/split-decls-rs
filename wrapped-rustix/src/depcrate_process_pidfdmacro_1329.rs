// Generated macro for macro_1329 (macro)
macro_rules! Depcrate_process_pidfdmacro_1329 {
() => {
// Module: crate::process::pidfd
// Provides: {"macro_1329"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `PIDFD_*` flags for use with [`pidfd_open`]."] # [doc = ""] # [doc = " [`pidfd_open`]: crate::process::pidfd_open"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct PidfdFlags : ffi :: c_uint { # [doc = " `PIDFD_NONBLOCK`"] const NONBLOCK = backend :: c :: PIDFD_NONBLOCK ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
