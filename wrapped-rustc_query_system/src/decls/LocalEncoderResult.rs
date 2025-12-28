macro_rules! LocalEncoderResult {
    () => {
        struct LocalEncoderResult { node_max : u32 , node_count : usize , edge_count : usize , # [doc = " Stores the number of times we've encoded each dep kind."] kind_stats : Vec < u32 > , }
    };
}

LocalEncoderResult!()