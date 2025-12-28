macro_rules! deps {
    () => {
        SuffixStore!();
    };
}

macro_rules! WindowEntry {
    () => {
        deps!();
        # [doc = " We keep a window of a few of these entries"] # [doc = " All of these are valid targets for a match to be generated for"] struct WindowEntry { data : Vec < u8 > , # [doc = " Stores indexes into data"] suffixes : SuffixStore , # [doc = " Makes offset calculations efficient"] base_offset : usize , }
    };
}

WindowEntry!()