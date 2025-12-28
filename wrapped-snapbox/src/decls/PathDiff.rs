macro_rules! deps {
    () => {
        Error!();
        FileType!();
        Data!();
    };
}

macro_rules! PathDiff {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum PathDiff { Failure (crate :: assert :: Error) , TypeMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_type : FileType , actual_type : FileType , } , LinkMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_target : std :: path :: PathBuf , actual_target : std :: path :: PathBuf , } , ContentMismatch { expected_path : std :: path :: PathBuf , actual_path : std :: path :: PathBuf , expected_content : crate :: Data , actual_content : crate :: Data , } , }
    };
}

PathDiff!();