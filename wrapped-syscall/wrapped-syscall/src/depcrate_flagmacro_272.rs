// Generated macro for macro_272 (macro)
macro_rules! Depcrate_flagmacro_272 {
() => {
// Module: crate::flag
// Provides: {"macro_272"}
// Dependencies: {}
bitflags :: bitflags ! { # [derive (Clone , Copy , Debug)] pub struct SendFdFlags : usize { # [doc = " If set, the kernel will enforce that the file descriptors are exclusively owned."] # [doc = ""] # [doc = " That is, there will no longer exist any other reference to those FDs when removed from"] # [doc = " the file table (sendfd always removes the FDs from the file table, but without this"] # [doc = " flag, it can be retained by SYS_DUPing them first)."] const EXCLUSIVE = 1 ; # [doc = " If set, the file descriptors will be cloned and *not* removed from the sender's file table."] # [doc = " By default, `SYS_SENDFD` moves the file descriptors, removing them from the sender."] const CLONE = 2 ; } }
};
}
