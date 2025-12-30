// Generated macro for impl_1396 (impl)
macro_rules! Depcrate_os_darwin_fsimpl_1396 {
() => {
// Module: crate::os::darwin::fs
// Provides: {"impl_1396"}
// Dependencies: {}
# [stable (feature = "file_set_times" , since = "1.75.0")] impl FileTimesExt for fs :: FileTimes { fn set_created (mut self , t : SystemTime) -> Self { self . as_inner_mut () . set_created (t . into_inner ()) ; self } }
};
}
