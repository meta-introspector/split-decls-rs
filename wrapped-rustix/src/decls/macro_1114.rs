macro_rules! deps {
    () => {
        Wait!();
    };
}

macro_rules! macro_1114 {
    () => {
        deps!();
        # [cfg (not (any (target_os = "horizon" , target_os = "openbsd" , target_os = "redox" , target_os = "wasi")))] bitflags ! { # [doc = " Options for modifying the behavior of [`waitid`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct WaitIdOptions : u32 { # [doc = " Return immediately if no child has exited."] const NOHANG = bitcast ! (backend :: process :: wait :: WNOHANG) ; # [doc = " Return if a stopped child has been resumed by delivery of"] # [doc = " [`Signal::Cont`]."] # [doc = ""] # [doc = " [`Signal::Cont`]: crate::process::Signal::Cont"] const CONTINUED = bitcast ! (backend :: process :: wait :: WCONTINUED) ; # [doc = " Wait for processed that have exited."] # [cfg (not (target_os = "cygwin"))] const EXITED = bitcast ! (backend :: process :: wait :: WEXITED) ; # [doc = " Keep processed in a waitable state."] # [cfg (not (target_os = "cygwin"))] const NOWAIT = bitcast ! (backend :: process :: wait :: WNOWAIT) ; # [doc = " Wait for processes that have been stopped."] # [cfg (not (target_os = "cygwin"))] const STOPPED = bitcast ! (backend :: process :: wait :: WSTOPPED) ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1114!();