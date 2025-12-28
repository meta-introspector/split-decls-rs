macro_rules! deps {
    () => {
        EndianMode!();
        Result!();
    };
}

macro_rules! impl_953 {
    () => {
        deps!();
        impl TryFrom < u32 > for EndianMode { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_ENDIAN_BIG => Ok (Self :: Big) , PR_ENDIAN_LITTLE => Ok (Self :: Little) , PR_ENDIAN_PPC_LITTLE => Ok (Self :: PowerPCLittle) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_953!();