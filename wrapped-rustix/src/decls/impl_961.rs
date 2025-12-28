macro_rules! deps {
    () => {
        Result!();
        TimeStampCounterReadability!();
    };
}

macro_rules! impl_961 {
    () => {
        deps!();
        impl TryFrom < u32 > for TimeStampCounterReadability { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_TSC_ENABLE => Ok (Self :: Readable) , PR_TSC_SIGSEGV => Ok (Self :: RaiseSIGSEGV) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_961!()