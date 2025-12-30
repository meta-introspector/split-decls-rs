// Generated macro for macro_122 (macro)
macro_rules! Depcrate_event_kqueuemacro_122 {
() => {
// Module: crate::event::kqueue
// Provides: {"macro_122"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The flags for a `kqueue` event specifying actions to perform."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct EventFlags : u16 { # [doc = " Add the event to the `kqueue`."] const ADD = c :: EV_ADD as _ ; # [doc = " Enable the event."] const ENABLE = c :: EV_ENABLE as _ ; # [doc = " Disable the event."] const DISABLE = c :: EV_DISABLE as _ ; # [doc = " Delete the event from the `kqueue`."] const DELETE = c :: EV_DELETE as _ ; # [doc = " TODO"] const RECEIPT = c :: EV_RECEIPT as _ ; # [doc = " Clear the event after it is triggered."] const ONESHOT = c :: EV_ONESHOT as _ ; # [doc = " TODO"] const CLEAR = c :: EV_CLEAR as _ ; # [doc = " TODO"] const EOF = c :: EV_EOF as _ ; # [doc = " TODO"] const ERROR = c :: EV_ERROR as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
