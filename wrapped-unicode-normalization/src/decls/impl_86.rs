macro_rules! deps {
    () => {
        StreamSafe!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > Iterator for StreamSafe < I > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { let next_ch = self . buffer . take () . or_else (| | self . iter . next ()) ? ; let d = classify_nonstarters (next_ch) ; if self . nonstarter_count + d . leading_nonstarters > MAX_NONSTARTERS { self . nonstarter_count = 0 ; self . buffer = Some (next_ch) ; return Some (COMBINING_GRAPHEME_JOINER) ; } if d . leading_nonstarters == d . decomposition_len { self . nonstarter_count += d . decomposition_len ; } else { self . nonstarter_count = d . trailing_nonstarters ; } Some (next_ch) } }
    };
}

impl_86!()