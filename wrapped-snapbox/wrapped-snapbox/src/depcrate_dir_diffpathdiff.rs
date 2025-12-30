// Generated macro for PathDiff (enum)
macro_rules! Depcrate_dir_diffPathDiff {
() => {
// Module: crate::dir::diff
// Provides: {"PathDiff"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub enum PathDiff { Failure (crate :: assert :: Error) , TypeMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_type : FileType , actual_type : FileType , } , LinkMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_target : std :: path :: PathBuf , actual_target : std :: path :: PathBuf , } , ContentMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_content : crate :: Data , actual_content : crate :: Data , } , }
};
}
