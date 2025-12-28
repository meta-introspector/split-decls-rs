macro_rules! deps {
    () => {
        Pair!();
        End!();
        Punctuated!();
        IntoPairs!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        impl < T , P > DoubleEndedIterator for IntoPairs < T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . map (Pair :: End) . or_else (| | self . inner . next_back () . map (| (t , p) | Pair :: Punctuated (t , p))) } }
    };
}

impl_627!()