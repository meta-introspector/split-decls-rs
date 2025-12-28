macro_rules! deps {
    () => {
        IndicesIter!();
        UnicodeWordIndices!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > Iterator for UnicodeWordIndices < 'a > { type Item = (usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match & mut self . inner { IndicesIter :: Ascii (i) => i . next () , IndicesIter :: Unicode (i) => i . next () , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match & self . inner { IndicesIter :: Ascii (i) => i . size_hint () , IndicesIter :: Unicode (i) => i . size_hint () , } } }
    };
}

impl_48!();