macro_rules! Segment {
    () => {
        pub struct Segment { # [doc = " The actual contents of the segment."] pub raw : Vec < u8 > , # [doc = " A measure of how \"ideal\" a given segment would be to include in the dictionary"] # [doc = ""] # [doc = " Higher is better, there's no upper limit. This number is determined by"] # [doc = " estimating the number of occurances in a given epoch"] pub score : usize , }
    };
}

Segment!();