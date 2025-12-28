macro_rules! deps {
    () => {
        End!();
        Punctuated!();
        Pair!();
        Pairs!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl < 'a , T , P > DoubleEndedIterator for Pairs < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . map (Pair :: End) . or_else (| | self . inner . next_back () . map (| (t , p) | Pair :: Punctuated (t , p))) } }
    };
}

impl_618!();