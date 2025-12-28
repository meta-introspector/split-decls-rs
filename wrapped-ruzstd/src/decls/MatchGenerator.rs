macro_rules! deps {
    () => {
        WindowEntry!();
    };
}

macro_rules! MatchGenerator {
    () => {
        deps!();
        pub (crate) struct MatchGenerator { max_window_size : usize , # [doc = " Data window we are operating on to find matches"] # [doc = " The data we want to find matches for is in the last slice"] window : Vec < WindowEntry > , window_size : usize , # [cfg (debug_assertions)] concat_window : Vec < u8 > , # [doc = " Index in the last slice that we already processed"] suffix_idx : usize , # [doc = " Gets updated when a new sequence is returned to point right behind that sequence"] last_idx_in_sequence : usize , }
    };
}

MatchGenerator!()