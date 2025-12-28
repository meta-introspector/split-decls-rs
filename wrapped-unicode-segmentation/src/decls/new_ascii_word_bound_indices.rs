macro_rules! deps {
    () => {
        AsciiWordBoundIter!();
    };
}

macro_rules! new_ascii_word_bound_indices {
    () => {
        deps!();
        # [inline] fn new_ascii_word_bound_indices (s : & str) -> AsciiWordBoundIter < '_ > { AsciiWordBoundIter :: new (s) }
    };
}

new_ascii_word_bound_indices!();