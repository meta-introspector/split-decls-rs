macro_rules! HEAP_SIZE {
    () => {
        # [doc = " maximum heap size"] const HEAP_SIZE : usize = 2 * L_CODES + 1 ;
    };
}

HEAP_SIZE!()