macro_rules! MAX_RET_IN_REGS_SIZE {
    () => {
        const MAX_RET_IN_REGS_SIZE : u64 = NUM_RET_GPRS * 32 ;
    };
}

MAX_RET_IN_REGS_SIZE!()