// Generated macro for macro_125 (macro)
macro_rules! Depcrate_event_kqueuemacro_125 {
() => {
// Module: crate::event::kqueue
// Provides: {"macro_125"}
// Dependencies: {}
# [cfg (any (apple , freebsdlike))] bitflags :: bitflags ! { # [doc = " The flags for a user event."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct UserFlags : u32 { # [doc = " Ignore the user input flags."] # [doc (alias = "NOP")] const NOINPUT = c :: NOTE_FFNOP ; # [doc = " Bitwise AND `fflags`."] const AND = c :: NOTE_FFAND ; # [doc = " Bitwise OR `fflags`."] const OR = c :: NOTE_FFOR ; # [doc = " Copy `fflags`."] const COPY = c :: NOTE_FFCOPY ; # [doc = " Control mask for operations."] const CTRLMASK = c :: NOTE_FFCTRLMASK ; # [doc = " User defined flags for masks."] const UDFMASK = c :: NOTE_FFLAGSMASK ; # [doc = " Trigger the event."] const TRIGGER = c :: NOTE_TRIGGER ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
