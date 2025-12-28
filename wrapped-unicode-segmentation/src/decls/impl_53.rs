macro_rules! deps {
    () => {
        UWordBoundIndices!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > Iterator for UWordBoundIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , & 'a str) > { self . iter . next () . map (| s | (s . as_ptr () as usize - self . start_offset , s)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_53!()