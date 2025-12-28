macro_rules! deps {
    () => {
        PrivateIterMut!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl < 'a , T , P > DoubleEndedIterator for PrivateIterMut < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . or_else (| | self . inner . next_back () . map (| pair | & mut pair . 0)) } }
    };
}

impl_658!()