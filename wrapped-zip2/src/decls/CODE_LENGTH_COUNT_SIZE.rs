macro_rules! CODE_LENGTH_COUNT_SIZE {
    () => {
        # [doc = " Size of the code length count array (lengths are actually 1..=16)"] const CODE_LENGTH_COUNT_SIZE : usize = MAX_CODE_LENGTH ;
    };
}

CODE_LENGTH_COUNT_SIZE!();