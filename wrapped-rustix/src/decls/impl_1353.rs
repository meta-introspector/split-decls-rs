macro_rules! deps {
    () => {
        SysCallUserDispatchFastSwitch!();
        Result!();
    };
}

macro_rules! impl_1353 {
    () => {
        deps!();
        impl TryFrom < u8 > for SysCallUserDispatchFastSwitch { type Error = io :: Errno ; fn try_from (value : u8) -> Result < Self , Self :: Error > { match value { SYSCALL_DISPATCH_FILTER_ALLOW => Ok (Self :: Allow) , SYSCALL_DISPATCH_FILTER_BLOCK => Ok (Self :: Block) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_1353!()