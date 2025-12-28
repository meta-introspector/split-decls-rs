macro_rules! MAX_WINDOW_SIZE {
    () => {
        # [doc = " Window size refers to the minimum amount of memory needed to decode any given frame."] # [doc = ""] # [doc = " The maximum window size allowed by the spec is 3.75TB"] pub const MAX_WINDOW_SIZE : u64 = (1 << 41) + 7 * (1 << 38) ;
    };
}

MAX_WINDOW_SIZE!()