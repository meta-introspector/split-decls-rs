// Generated macro for macro_123 (macro)
macro_rules! Depcrate_event_kqueuemacro_123 {
() => {
// Module: crate::event::kqueue
// Provides: {"macro_123"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The flags for a virtual node event."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct VnodeEvents : u32 { # [doc = " The file was deleted."] const DELETE = c :: NOTE_DELETE ; # [doc = " The file was written to."] const WRITE = c :: NOTE_WRITE ; # [doc = " The file was extended."] const EXTEND = c :: NOTE_EXTEND ; # [doc = " The file had its attributes changed."] const ATTRIBUTES = c :: NOTE_ATTRIB ; # [doc = " The file was renamed."] const RENAME = c :: NOTE_RENAME ; # [doc = " Access to the file was revoked."] const REVOKE = c :: NOTE_REVOKE ; # [doc = " The link count of the file has changed."] const LINK = c :: NOTE_LINK ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
