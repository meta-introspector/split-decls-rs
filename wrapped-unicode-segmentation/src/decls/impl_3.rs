macro_rules! deps {
    () => {
        GraphemeIndices!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'a > Iterator for GraphemeIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , & 'a str) > { self . iter . next () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_3!()