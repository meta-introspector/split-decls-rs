// Generated macro for impl_752 (impl)
macro_rules! Depcrate_fsimpl_752 {
() => {
// Module: crate::fs
// Provides: {"impl_752"}
// Dependencies: {}
impl FileTimes { # [doc = " Creates a new `FileTimes` with no times set."] # [doc = ""] # [doc = " Using the resulting `FileTimes` in [`File::set_times`] will not modify any timestamps."] # [stable (feature = "file_set_times" , since = "1.75.0")] pub fn new () -> Self { Self :: default () } # [doc = " Set the last access time of a file."] # [stable (feature = "file_set_times" , since = "1.75.0")] pub fn set_accessed (mut self , t : SystemTime) -> Self { self . 0 . set_accessed (t . into_inner ()) ; self } # [doc = " Set the last modified time of a file."] # [stable (feature = "file_set_times" , since = "1.75.0")] pub fn set_modified (mut self , t : SystemTime) -> Self { self . 0 . set_modified (t . into_inner ()) ; self } }
};
}
