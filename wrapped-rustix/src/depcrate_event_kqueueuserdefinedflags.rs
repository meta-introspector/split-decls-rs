// Generated macro for UserDefinedFlags (struct)
macro_rules! Depcrate_event_kqueueUserDefinedFlags {
() => {
// Module: crate::event::kqueue
// Provides: {"UserDefinedFlags"}
// Dependencies: {}
# [doc = " User-defined flags."] # [doc = ""] # [doc = " Only the lower 24 bits are used in this struct."] # [repr (transparent)] # [cfg (any (apple , freebsdlike))] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct UserDefinedFlags (u32) ;
};
}
