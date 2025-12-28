macro_rules! deps {
    () => {
        GraphemeIndices!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for GraphemeIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , & 'a str) > { self . iter . next_back () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } }
    };
}

impl_4!();