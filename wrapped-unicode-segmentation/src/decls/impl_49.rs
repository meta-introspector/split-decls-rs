macro_rules! deps {
    () => {
        UnicodeWordIndices!();
        IndicesIter!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for UnicodeWordIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { match & mut self . inner { IndicesIter :: Ascii (i) => i . next_back () , IndicesIter :: Unicode (i) => i . next_back () , } } }
    };
}

impl_49!()