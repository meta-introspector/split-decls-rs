macro_rules! MAXIMUM_ALLOWED_WINDOW_SIZE {
    () => {
        # [doc = " While the maximum window size allowed by the spec is significantly larger,"] # [doc = " our implementation limits it to 100mb to protect against malformed frames."] const MAXIMUM_ALLOWED_WINDOW_SIZE : u64 = 1024 * 1024 * 100 ;
    };
}

MAXIMUM_ALLOWED_WINDOW_SIZE!();