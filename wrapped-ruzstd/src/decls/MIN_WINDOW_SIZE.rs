macro_rules! MIN_WINDOW_SIZE {
    () => {
        # [doc = " Window size refers to the minimum amount of memory needed to decode any given frame."] # [doc = ""] # [doc = " The minimum window size is defined as 1 KB"] pub const MIN_WINDOW_SIZE : u64 = 1024 ;
    };
}

MIN_WINDOW_SIZE!();