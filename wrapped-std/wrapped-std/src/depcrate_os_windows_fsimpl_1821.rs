// Generated macro for impl_1821 (impl)
macro_rules! Depcrate_os_windows_fsimpl_1821 {
() => {
// Module: crate::os::windows::fs
// Provides: {"impl_1821"}
// Dependencies: {}
# [stable (feature = "file_set_times" , since = "1.75.0")] impl FileTimesExt for fs :: FileTimes { fn set_created (mut self , t : SystemTime) -> Self { self . as_inner_mut () . set_created (t . into_inner ()) ; self } }
};
}
