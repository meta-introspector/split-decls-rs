macro_rules! StreamSafe {
    () => {
        # [doc = " [UAX15-D4]: This iterator keeps track of how many non-starters there have been"] # [doc = " since the last starter in *NFKD* and will emit a Combining Grapheme Joiner"] # [doc = " (U+034F) if the count exceeds 30."] # [doc = ""] # [doc = " [UAX15-D4]: https://www.unicode.org/reports/tr15/#UAX15-D4"] pub struct StreamSafe < I > { iter : I , nonstarter_count : usize , buffer : Option < char > , }
    };
}

StreamSafe!();