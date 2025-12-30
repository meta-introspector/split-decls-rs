// Generated macro for macro_1584 (macro)
macro_rules! Depcrate_process_waitmacro_1584 {
() => {
// Module: crate::process::wait
// Provides: {"macro_1584"}
// Dependencies: {}
bitflags ! { # [doc = " Options for modifying the behavior of [`wait`]/[`waitpid`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct WaitOptions : u32 { # [doc = " Return immediately if no child has exited."] const NOHANG = bitcast ! (backend :: process :: wait :: WNOHANG) ; # [doc = " Return if a child has stopped (but not traced via [`ptrace`])."] # [doc = ""] # [doc = " [`ptrace`]: https://man7.org/linux/man-pages/man2/ptrace.2.html"] # [cfg (not (target_os = "horizon"))] const UNTRACED = bitcast ! (backend :: process :: wait :: WUNTRACED) ; # [doc = " Return if a stopped child has been resumed by delivery of"] # [doc = " [`Signal::Cont`]."] # [doc = ""] # [doc = " [`Signal::Cont`]: crate::process::Signal::Cont"] # [cfg (not (target_os = "horizon"))] const CONTINUED = bitcast ! (backend :: process :: wait :: WCONTINUED) ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
