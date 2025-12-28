macro_rules! LocalEncoderState {
    () => {
        struct LocalEncoderState { next_node_index : u32 , remaining_node_index : u32 , encoder : MemEncoder , node_count : usize , edge_count : usize , # [doc = " Stores the number of times we've encoded each dep kind."] kind_stats : Vec < u32 > , }
    };
}

LocalEncoderState!();