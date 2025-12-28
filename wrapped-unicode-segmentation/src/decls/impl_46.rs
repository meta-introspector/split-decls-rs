macro_rules! deps {
    () => {
        UnicodeWords!();
        WordsIter!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for UnicodeWords < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { match & mut self . inner { WordsIter :: Ascii (i) => i . next_back () , WordsIter :: Unicode (i) => i . next_back () , } } }
    };
}

impl_46!()