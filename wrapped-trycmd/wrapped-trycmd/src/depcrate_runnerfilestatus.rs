// Generated macro for FileStatus (enum)
macro_rules! Depcrate_runnerFileStatus {
() => {
// Module: crate::runner
// Provides: {"FileStatus"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] enum FileStatus { Ok { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , } , Failure (crate :: Error) , TypeMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_type : FileType , actual_type : FileType , } , LinkMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_target : std :: path :: PathBuf , actual_target : std :: path :: PathBuf , } , ContentMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_content : crate :: Data , actual_content : crate :: Data , } , }
};
}
