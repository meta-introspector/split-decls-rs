// Generated macro for ThreadInner (struct)
macro_rules! Depcrate_threadThreadInner {
() => {
// Module: crate::thread
// Provides: {"ThreadInner"}
// Dependencies: {}
# [doc = " Inner shared wrapper for [`Thread`]."] # [derive (Debug)] struct ThreadInner { # [doc = " [`ThreadId`]."] id : ThreadId , # [doc = " Name of the thread."] name : Option < String > , # [doc = " Parker implementation."] parker : Parker , }
};
}
