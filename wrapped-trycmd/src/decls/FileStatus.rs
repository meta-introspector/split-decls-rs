macro_rules! FileStatus {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] enum FileStatus { Ok { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , } , Failure (crate :: Error) , TypeMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_type : FileType , actual_type : FileType , } , LinkMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_target : std :: path :: PathBuf , actual_target : std :: path :: PathBuf , } , ContentMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_content : crate :: Data , actual_content : crate :: Data , } , }
    };
}

FileStatus!();