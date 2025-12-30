// Generated macro for JoinInner (struct)
macro_rules! Depcrate_threadJoinInner {
() => {
// Module: crate::thread
// Provides: {"JoinInner"}
// Dependencies: {}
# [doc = " Inner representation for JoinHandle"] struct JoinInner < 'scope , T > { native : imp :: Thread , thread : Thread , packet : Arc < Packet < 'scope , T > > , }
};
}
