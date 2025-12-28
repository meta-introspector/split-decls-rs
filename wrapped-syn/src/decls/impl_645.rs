macro_rules! deps {
    () => {
        PrivateIter!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl < 'a , T , P > DoubleEndedIterator for PrivateIter < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . or_else (| | self . inner . next_back () . map (| pair | & pair . 0)) } }
    };
}

impl_645!();