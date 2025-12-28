macro_rules! deps {
    () => {
        Result!();
        MachineCheckMemoryCorruptionKillPolicy!();
    };
}

macro_rules! impl_973 {
    () => {
        deps!();
        impl TryFrom < u32 > for MachineCheckMemoryCorruptionKillPolicy { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_MCE_KILL_LATE => Ok (Self :: Late) , PR_MCE_KILL_EARLY => Ok (Self :: Early) , PR_MCE_KILL_DEFAULT => Ok (Self :: Default) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_973!();