macro_rules! DEF_MEM_LEVEL {
    () => {
        const DEF_MEM_LEVEL : i32 = if MAX_MEM_LEVEL > 8 { 8 } else { MAX_MEM_LEVEL } ;
    };
}

DEF_MEM_LEVEL!()