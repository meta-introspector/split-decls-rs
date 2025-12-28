macro_rules! deps {
    () => {
        UWordBoundIndices!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for UWordBoundIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , & 'a str) > { self . iter . next_back () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } }
    };
}

impl_54!()