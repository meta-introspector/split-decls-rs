macro_rules! deps {
    () => {
        UWordBounds!();
    };
}

macro_rules! new_word_bounds {
    () => {
        deps!();
        # [inline] pub fn new_word_bounds (s : & str) -> UWordBounds < '_ > { UWordBounds { string : s , cat : None , catb : None , } }
    };
}

new_word_bounds!();