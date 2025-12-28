macro_rules! deps {
    () => {
        DumpableBehavior!();
        Result!();
    };
}

macro_rules! impl_921 {
    () => {
        deps!();
        impl TryFrom < i32 > for DumpableBehavior { type Error = io :: Errno ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { SUID_DUMP_DISABLE => Ok (Self :: NotDumpable) , SUID_DUMP_USER => Ok (Self :: Dumpable) , SUID_DUMP_ROOT => Ok (Self :: DumpableReadableOnlyByRoot) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_921!();