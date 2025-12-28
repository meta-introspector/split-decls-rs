macro_rules! deps {
    () => {
        Result!();
        SecureComputingMode!();
    };
}

macro_rules! impl_1289 {
    () => {
        deps!();
        impl TryFrom < i32 > for SecureComputingMode { type Error = io :: Errno ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { SECCOMP_MODE_DISABLED => Ok (Self :: Disabled) , SECCOMP_MODE_STRICT => Ok (Self :: Strict) , SECCOMP_MODE_FILTER => Ok (Self :: Filter) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_1289!()