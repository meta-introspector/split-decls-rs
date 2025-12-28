macro_rules! MAX_ARG_IN_REGS_SIZE {
    () => {
        const MAX_ARG_IN_REGS_SIZE : u64 = NUM_ARG_GPRS * 32 ;
    };
}

MAX_ARG_IN_REGS_SIZE!();