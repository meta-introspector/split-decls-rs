macro_rules! deps {
    () => {
        Result!();
        TimingMethod!();
    };
}

macro_rules! impl_944 {
    () => {
        deps!();
        impl TryFrom < i32 > for TimingMethod { type Error = io :: Errno ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { PR_TIMING_STATISTICAL => Ok (Self :: Statistical) , PR_TIMING_TIMESTAMP => Ok (Self :: TimeStamp) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_944!();