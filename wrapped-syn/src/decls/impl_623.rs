macro_rules! deps {
    () => {
        Punctuated!();
        PairsMut!();
        Pair!();
        End!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        impl < 'a , T , P > DoubleEndedIterator for PairsMut < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . map (Pair :: End) . or_else (| | self . inner . next_back () . map (| (t , p) | Pair :: Punctuated (t , p))) } }
    };
}

impl_623!()