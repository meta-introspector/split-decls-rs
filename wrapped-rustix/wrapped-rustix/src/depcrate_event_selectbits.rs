// Generated macro for BITS (const)
macro_rules! Depcrate_event_selectBITS {
() => {
// Module: crate::event::select
// Provides: {"BITS"}
// Dependencies: {}
# [cfg (not (any (windows , target_os = "wasi")))] const BITS : usize = size_of :: < FdSetElement > () * 8 ;
};
}
