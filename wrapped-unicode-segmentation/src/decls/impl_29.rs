macro_rules! deps {
    () => {
        UnicodeSentences!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a > Iterator for UnicodeSentences < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_29!();