// Generated macro for SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME (const)
macro_rules! Depcrate_lib_generatedSUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME {
() => {
// Module: crate::lib_generated
// Provides: {"SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME"}
// Dependencies: {}
# [doc = " If set, treat the timestamp provided in"] # [doc = " `subscription_clock::timeout` as an absolute timestamp of clock"] # [doc = " `subscription_clock::id`. If clear, treat the timestamp"] # [doc = " provided in `subscription_clock::timeout` relative to the"] # [doc = " current time value of clock `subscription_clock::id`."] pub const SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME : Subclockflags = 1 << 0 ;
};
}
