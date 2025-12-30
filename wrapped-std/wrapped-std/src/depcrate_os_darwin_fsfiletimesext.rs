// Generated macro for FileTimesExt (trait)
macro_rules! Depcrate_os_darwin_fsFileTimesExt {
() => {
// Module: crate::os::darwin::fs
// Provides: {"FileTimesExt"}
// Dependencies: {}
# [doc = " OS-specific extensions to [`fs::FileTimes`]."] # [stable (feature = "file_set_times" , since = "1.75.0")] pub trait FileTimesExt : Sealed { # [doc = " Set the creation time of a file."] # [stable (feature = "file_set_times" , since = "1.75.0")] fn set_created (self , t : SystemTime) -> Self ; }
};
}
