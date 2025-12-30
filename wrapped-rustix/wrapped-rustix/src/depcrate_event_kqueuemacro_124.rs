// Generated macro for macro_124 (macro)
macro_rules! Depcrate_event_kqueuemacro_124 {
() => {
// Module: crate::event::kqueue
// Provides: {"macro_124"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The flags for a process event."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct ProcessEvents : u32 { # [doc = " The process exited."] const EXIT = c :: NOTE_EXIT ; # [doc = " The process forked itself."] const FORK = c :: NOTE_FORK ; # [doc = " The process executed a new process."] const EXEC = c :: NOTE_EXEC ; # [doc = " Follow the process through `fork` calls (write only)."] const TRACK = c :: NOTE_TRACK ; # [doc = " An error has occurred with following the process."] const TRACKERR = c :: NOTE_TRACKERR ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
