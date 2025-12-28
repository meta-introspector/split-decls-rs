macro_rules! deps {
    () => {
        REG_ROUTINE_FLAGS!();
    };
}

macro_rules! RRF_RT_REG_DWORD {
    () => {
        deps!();
        pub const RRF_RT_REG_DWORD : REG_ROUTINE_FLAGS = 16u32 ;
    };
}

RRF_RT_REG_DWORD!()