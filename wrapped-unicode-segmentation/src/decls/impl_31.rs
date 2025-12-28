macro_rules! deps {
    () => {
        USentenceBoundIndices!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a > Iterator for USentenceBoundIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , & 'a str) > { self . iter . next () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_31!()