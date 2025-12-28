macro_rules! MAX_CODE {
    () => {
        const MAX_CODE : usize = (1 << MAX_CODE_SIZE) - 1 ;
    };
}

MAX_CODE!()