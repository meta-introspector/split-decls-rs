macro_rules! deps {
    () => {
        FileStatus!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl From < snapbox :: dir :: PathDiff > for FileStatus { fn from (other : snapbox :: dir :: PathDiff) -> Self { match other { snapbox :: dir :: PathDiff :: Failure (err) => FileStatus :: Failure (err) , snapbox :: dir :: PathDiff :: TypeMismatch { expected_path , actual_path , expected_type , actual_type , } => FileStatus :: TypeMismatch { actual_path , expected_path , actual_type , expected_type , } , snapbox :: dir :: PathDiff :: LinkMismatch { expected_path , actual_path , expected_target , actual_target , } => FileStatus :: LinkMismatch { actual_path , expected_path , actual_target , expected_target , } , snapbox :: dir :: PathDiff :: ContentMismatch { expected_path , actual_path , expected_content , actual_content , } => FileStatus :: ContentMismatch { actual_path , expected_path , actual_content , expected_content , } , } } }
    };
}

impl_76!()